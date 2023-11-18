mod current;
pub use current::*;

mod forecast;
pub use forecast::*;

use serde::{Serialize, Deserialize};
pub use chrono::{DateTime, TimeZone, FixedOffset, ParseResult, NaiveDate, NaiveDateTime, Local};
use std::fmt::{self, Display};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct History {
    pub location: Location,
    pub forecast: _Forecast,
}

pub type Future = History;


pub trait Date {
    fn _date_from_str(&self, s: &str) -> ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
    }

    fn date(&self) -> NaiveDate;
    fn date_epoch(&self) -> NaiveDate;
}


pub trait Time {
    fn _time_from_str(&self, s: &str) -> ParseResult<NaiveDateTime> {
        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M")
    }

    fn _local_time_from_str(&self, s: &str) -> ParseResult<DateTime<Local>> {
        let dt = self._time_from_str(s)?;
        Ok(Local.from_local_datetime(&dt).earliest().unwrap())
    }

    fn time(&self) -> DateTime<Local>;
    fn time_epoch(&self) -> DateTime<Local>;
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AirQuality {
    #[serde(rename = "co")]
    pub carbon_monoxide: String,
    #[serde(rename = "o3")]
    pub ozone: String,
    #[serde(rename = "no2")]
    pub nitrogen_dioxide: String,
    #[serde(rename = "so2")]
    pub sulphur_dioxide: String,
    pub pm2_5: String,
    pub pm10: String,
    #[serde(rename = "us-epa-index")]
    pub us_epa_index: u32,
    #[serde(rename = "gb-defra-index")]
    pub gb_defra_index: u32
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: u32
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WeatherAlert {
    #[serde(rename = "headline")]
    pub head_line: String,
    #[serde(rename = "msgType")]
    pub msg_type: String,
    pub severity: String,
    pub urgency: String,
    pub areas: String,
    pub category: String,
    pub certainty: String,
    pub event: String,
    pub note: String,
    pub effective: String,
    pub expires: String,
    #[serde(rename = "desc")]
    pub description: String,
    pub instruction: String
}

impl WeatherAlert {
    pub fn effective(&self) -> DateTime<FixedOffset> {
        self.effective.parse().unwrap()
    }

    pub fn expires(&self) -> DateTime<FixedOffset> {
        self.expires.parse().unwrap()
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Alerts {
    pub alert: Vec<WeatherAlert>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Coords {
    pub lat: f32,
    pub lon: f32
}

impl Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.lat, self.lon)
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(flatten)]
    pub coords: Coords,
    pub name: String,
    pub region: String,
    pub country: String,
    pub id: Option<u32>,
    pub url: Option<String>,
    pub tz_id: Option<String>,
    pub localtime_epoch: Option<i64>,
    pub localtime: Option<String>
}

impl Time for Location {
    fn time(&self) -> DateTime<Local> {
        self._local_time_from_str(self.localtime.as_ref().unwrap()).unwrap()
    }

    fn time_epoch(&self) -> DateTime<Local> {
        Local.timestamp_opt(self.localtime_epoch.unwrap(), 0).unwrap()
    }
}
