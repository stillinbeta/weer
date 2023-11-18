use std::fmt::Display;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, TimeZone, NaiveTime, Local, NaiveDate};

use super::{Current, Location, Alerts, AirQuality, Condition, Date, Time};


pub struct Temperature(f32, f32);

impl Temperature {
    pub fn min(&self) -> f32 { self.0 }
    pub fn max(&self) -> f32 { self.1 }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "min: {}\n max: {}", self.min(), self.max())
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Day {
    maxtemp_c: f32,
    maxtemp_f: f32,
    mintemp_c: f32,
    mintemp_f: f32,
    pub avgtemp_c: f32,
    pub avgtemp_f: f32,
    pub maxwind_mph: f32,
    pub maxwind_kph: f32,
    pub totalprecip_mm: f32,
    pub totalprecip_in: f32,
    pub avgvis_km: f32,
    pub avgvis_miles: f32,
    pub avghumidity: f32,
    pub condition: Condition,
    pub uv: f32
}

impl Day {
    pub fn temp_c(&self) -> Temperature {
        Temperature(self.mintemp_c, self.maxtemp_c)
    }

    pub fn temp_f(&self) -> Temperature {
        Temperature(self.mintemp_f, self.maxtemp_f)
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MoonPhase {
    #[serde(rename = "New Moon")]
    NewMoon,
    #[serde(rename = "Waxing Crescent")]
    WaxingCrescent,
    #[serde(rename = "First Quarter")]
    FirstQuarter,
    #[serde(rename = "Waxing Gibbous")]
    WaxingGibbous,
    #[serde(rename = "Full Moon")]
    FullMoon,
    #[serde(rename = "Waning Gibbous")]
    WaningGibbous,
    #[serde(rename = "Last Quarter")]
    LastQuarter,
    #[serde(rename = "Waning Crescent")]
    WaningCrescent,
    #[serde(rename = "Third Quarter")]
    ThirdQuarter
}

impl Display for MoonPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoonPhase::NewMoon => write!(f, "New Moon"),
            MoonPhase::WaxingCrescent => write!(f, "Waxing Crescent"),
            MoonPhase::FirstQuarter => write!(f, "First Quarter"),
            MoonPhase::WaxingGibbous => write!(f, "Waxing Gibbous"),
            MoonPhase::FullMoon => write!(f, "Full Moon"),
            MoonPhase::WaningCrescent => write!(f, "Waning Crescent"),
            MoonPhase::LastQuarter => write!(f, "Last Quarter"),
            MoonPhase::WaningGibbous => write!(f, "Waning Gibbous"),
            MoonPhase::ThirdQuarter => write!(f, "Third Quarter")
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moon_phase: MoonPhase,
    pub moon_illumination: u8,
}

impl Astro {
    pub fn sunrise(&self) -> NaiveTime {
        NaiveTime::parse_from_str(&self.sunrise, "%m:%d %p").unwrap()
    }

    pub fn sunset(&self) -> NaiveTime {
        NaiveTime::parse_from_str(&self.sunrise, "%m:%d %p").unwrap()
    }

    pub fn moonrise(&self) -> NaiveTime {
        NaiveTime::parse_from_str(&self.sunrise, "%m:%d %p").unwrap()
    }

    pub fn moonset(&self) -> NaiveTime {
        NaiveTime::parse_from_str(&self.sunrise, "%m:%d %p").unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Hour {
    pub time_epoch: i64,
    pub time: String,
    pub temp_c: f32,
    pub temp_f: f32,
    pub condition: Condition,
    pub wind_mph: f32,
    pub wind_kph: f32,
    pub wind_degree: u32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub precip_mm: f32,
    pub precip_in: f32,
    pub humidity: u32,
    pub cloud: u32,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub windchill_c: f32,
    pub windchill_f: f32,
    pub heatindex_c: f32,
    pub heatindex_f: f32,
    pub dewpoint_c: f32,
    pub dewpoint_f: f32,
    pub will_it_rain: u8,
    pub will_it_snow: u8,
    pub is_day: u8,
    pub vis_km: f32,
    pub vis_miles: f32,
    pub chance_of_rain: u32,
    pub chance_of_snow: u32,
    pub gust_mph: f32,
    pub gust_kph: f32,
    pub air_quality: Option<AirQuality>
}

impl Hour {
    pub fn will_it_rain(&self) -> bool {
        self.will_it_rain == 1
    }

    pub fn will_it_snow(&self) -> bool {
        self.will_it_snow == 1
    }

    pub fn is_day(&self) -> bool {
        self.is_day == 1
    }
}

impl Time for Hour {
    fn time(&self) -> DateTime<Local> {
        self._local_time_from_str(&self.time).unwrap()
    }

    fn time_epoch(&self) -> DateTime<Local> {
        Local.timestamp_opt(self.time_epoch, 0).unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForecastDay {
    pub date: String,
    pub date_epoch: i64,
    pub day: Day,
    pub astro: Astro,
    pub hour: Vec<Hour>
}

impl Date for ForecastDay {
    fn date(&self) -> NaiveDate {
        self._date_from_str(&self.date).unwrap()
    }

    fn date_epoch(&self) -> NaiveDate {
        Local.timestamp_opt(self.date_epoch, 0).unwrap().date_naive()
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct _Forecast {
    #[serde(rename = "forecastday")]
    pub forecast_day: Vec<ForecastDay>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Forecast {
    pub location: Location,
    pub current: Current,
    pub forecast: _Forecast,
    pub alerts: Option<Alerts>
}
