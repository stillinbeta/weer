use std::{
    io::{self, Write}, 
    net::IpAddr, 
    error::Error
};

use weer_api::{*, chrono::{Utc, TimeZone, Timelike}};
use super::{App, cli::tables};
use clap::ArgMatches;
use colored::*;


pub fn matches_handler(app: &App) -> Result<(), Box<dyn Error>> {
    let out = io::stdout();
    // let err = io::stderr();

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
        Some(("history", _sub_m)) => Ok(()),
        Some(("search", _sub_m)) => Ok(()),
        Some(("future", _sub_m)) => Ok(()),
        Some((&_, _)) => todo!(),
        None => Ok(())
    }
}

fn forecast(sub_m: &ArgMatches, out: &io::Stdout, app: &App, q: Query) -> Result<(), Box<dyn Error>> {
    let mut out = out.lock();

    let days = sub_m.get_one::<String>("days").unwrap().parse::<u8>()?;
    let hour = sub_m.get_one::<String>("hour").unwrap_or(&"0".to_string()).parse::<u32>()?;
    let aqi = sub_m.get_one::<bool>("aqi").unwrap();
    let alerts = sub_m.get_one::<bool>("alerts").unwrap();
    let dt = if let Some(dt) = sub_m.get_one::<String>("dt") {
        Utc.datetime_from_str(dt, "%Y-%m-%d")?.with_hour(hour).unwrap()
    } else {
        Utc::now().with_hour(hour).unwrap()
    };

    let mut req = app.client.forecast()
        .query(q)
        .dt(dt)
        .days(days)
        .hour();

    if let Some(lang) = app.cfg.lang() {
        req = req.lang(lang);
    }

    if *aqi {
        req = req.aqi();
    }

    if *alerts {
        req = req.alerts();
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
