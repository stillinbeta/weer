use std::error::Error;
use weer_api::{Location, Current, ForecastDay, AirQuality};
use prettytable::{Table, table, row};
use super::image;


pub fn current_table(current: Current) -> Result<Table, Box<dyn Error>> {
    let mut table = table!(
        [image::convert(&current.condition.icon)?, current.condition.text],
        ["Last updated", current.last_updated],
        ["Temperature in celsius", current.temp_c],
        ["Temperature in fahrenheit", current.temp_f],
        // ["Condition", current.condition.text],
        ["Is day", current.is_day()],
        ["Humidity as percentage", format!("{}%", current.humidity)],
        ["Cloud cover as percentage", format!("{}%", current.cloud)]
    );

    if let Some(air) = current.air_quality {
        table.add_row(row!["Air quality", air_quality_table(air)]);
    }

    Ok(table)
}

pub fn forecastday_table(fd: &ForecastDay) -> Result<Table, Box<dyn Error>> {
    let day_table = table!(
        [image::convert(&fd.day.condition.icon)?, fd.day.condition.text],
        ["Temperature in celsius", fd.day.temp_c()],
        ["Temperature in fahrenheit", fd.day.temp_f()]
    );

    let astro_table = table!(
        ["Sunrise time", fd.astro.sunrise],
        ["Sunset time", fd.astro.sunset],
        ["Moonrise time", fd.astro.moonrise],
        ["Moonset time", fd.astro.moonset],
        ["Moon phase", fd.astro.moon_phase],
        ["Moon illumination as %", format!("{}%", fd.astro.moon_illumination)]
    );
    
    Ok(table!(["Date", fd.date], ["Day", day_table], ["Astro", astro_table]))
}

pub fn location_table(location: Location) -> Table {
    let mut table = table!(
        ["Name", location.name], 
        ["Country", location.country], 
        ["Latitude and Longitude", format!("{}", location.coords)]
    );

    if let Some(tz_id) = location.tz_id {
        table.add_row(row!["TimeZone ID", tz_id]);
    }

    if let Some(localtime) = location.localtime {
        table.add_row(row!["Localtime", localtime]);
    }

    table
}

pub fn air_quality_table(air: AirQuality) -> Table {
    table!(
        ["Carbon Monoxide (μg/m3)", air.carbon_monoxide],
        ["Ozone (μg/m3)", air.ozone],
        ["Nitrogen dioxide (μg/m3)", air.nitrogen_dioxide],
        ["Sulphur dioxide (μg/m3)", air.sulphur_dioxide],
        ["PM2.5 (μg/m3)", air.pm2_5],
        ["PM10 (μg/m3)", air.pm10],
        ["US - EPA standard", if air.us_epa_index <= 3 { "Low" } else if air.us_epa_index <= 6 { "Moderate" } else if air.us_epa_index <= 9 { "High" } else { "Very High" }]
    )
}
