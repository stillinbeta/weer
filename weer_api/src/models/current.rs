use serde::{Serialize, Deserialize};
use chrono::{DateTime, TimeZone, Utc};

use super::{Condition, AirQuality, Location};


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Current {
    pub last_updated: String,
    pub last_updated_epoch: i64,
    pub temp_c: f32,
    pub temp_f: f32,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub condition: Condition,
    pub wind_mph: f32,
    pub wind_kph: f32,
    pub wind_degree: f32,
    pub wind_dir: String,
    pub pressure_mb: f32,
    pub pressure_in: f32,
    pub precip_mm: f32,
    pub precip_in: f32,
    pub humidity: u32,
    pub cloud: u32,
    pub is_day: u8,
    pub uv: f32,
    pub gust_mph: f32,
    pub gust_kph: f32,
    pub air_quality: Option<AirQuality>
}

impl Current {
    pub fn last_updated(&self) -> DateTime<Utc> {
        Utc.datetime_from_str(&self.last_updated, "%Y-%m-%d %H:%M").unwrap()
    }

    pub fn last_updated_epoch(&self) -> DateTime<Utc> {
        Utc.timestamp(self.last_updated_epoch, 0)
    }

    pub fn is_day(&self) -> bool {
        match self.is_day {
            0 => false,
            1 => true,
            _ => unreachable!()
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Realtime {
    pub location: Location,
    pub current: Current
}
