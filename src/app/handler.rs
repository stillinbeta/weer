use std::{
    io::{self, Write}, 
    net::IpAddr, 
    error::Error
};

use weer_api::{*, chrono::{Utc, TimeZone, Timelike}};
use clap::ArgMatches;
use colored::*;
use super::{App, tables};


pub fn matches_handler(app: &App) -> Result<(), Box<dyn Error>> {
    let out = io::stdout();

    let q = {
        if let Some(ip) = app.matches.get_one::<IpAddr>("ip") {
            Query::Ip(Some(ip.clone()))
        } else if let Some(name) = app.matches.get_one::<String>("city") {
            Query::City(name.clone())
        } else if let Some((lat, lon)) = app.matches.get_one::<(f32, f32)>("coords") {
            Query::Coords(lat.clone(), lon.clone())
        } else {
            Query::Ip(None)
        }
    };

    match app.matches.subcommand() {
        Some(("forecast", sub_m)) => forecast(sub_m, &out, &app, q),
        Some(("history", sub_m)) => history(sub_m, &out, &app, q),
        // Some(("search", _sub_m)) => todo!(),
        // Some(("future", _sub_m)) => todo!(),
        Some((&_, _)) => todo!(),
        None => Ok(())
    }
}

fn forecast(sub_m: &ArgMatches, out: &io::Stdout, app: &App, q: Query) -> Result<(), Box<dyn Error>> {
    let mut out = out.lock();

    let days = sub_m.get_one::<u8>("days").unwrap();
    let hour = sub_m.get_one::<u32>("hour").unwrap_or(&0);
    let aqi = sub_m.get_one::<bool>("aqi").unwrap();
    let alerts = sub_m.get_one::<bool>("alerts").unwrap();

    let dt = if let Some(dt) = sub_m.get_one::<String>("dt") {
        Utc.datetime_from_str(dt, "%Y-%m-%d")?.with_hour(hour.clone()).unwrap()
    } else {
        Utc::now().with_hour(hour.clone()).unwrap()
    };

    let mut req = app.client.forecast();
    req.query(q)
        .dt(dt)
        .days(days.clone())
        .hour(true)
        .aqi(*aqi)
        .alerts(*alerts);

    if let Some(lang) = app.cfg.lang() {
        req.lang(lang);
    }

    let resp = req.call()?;

    writeln!(out, "{}:", "Location".bold())?;
    tables::location_table(resp.location).print(&mut out)?;

    writeln!(out, "{}:", "Current".bold())?;
    tables::current_table(resp.current)?.print(&mut out)?;
    
    for fd in resp.forecast.forecast_day.iter() {
        writeln!(out, "{} - {}:", "Forecast".bold(), fd.date.italic())?;
        let table = tables::forecastday_table(fd)?;
        table.print(&mut out)?;
    }

    Ok(())
}

fn history(sub_m: &ArgMatches, out: &io::Stdout, app: &App, q: Query) -> Result<(), Box<dyn Error>> {
    let mut out = out.lock();

    let hour = sub_m.get_one::<u32>("hour").unwrap_or(&0);
    let dt = if let Some(dt) = sub_m.get_one::<String>("dt") {
        Utc.datetime_from_str(dt, "%Y-%m-%d")?.with_hour(hour.clone()).unwrap()
    } else {
        Utc::now().with_hour(hour.clone()).unwrap()
    };

    let mut req = app.client.history();
    req.query(q)
        .dt(dt)
        .hour(true);
    
    if let Some(end_dt) = sub_m.get_one::<String>("end_dt") {
        req.end_dt(Utc.datetime_from_str(end_dt, "%Y-%m-%d")?);
    }

    let resp = req.call()?;

    writeln!(out, "{}:", "Location".bold())?;
    tables::location_table(resp.location).print(&mut out)?;

    for fd in resp.forecast.forecast_day.iter() {
        writeln!(out, "{} - {}:", "Forecast".bold(), fd.date.italic())?;
        let table = tables::forecastday_table(fd)?;
        table.print(&mut out)?;
    }

    Ok(())
}
