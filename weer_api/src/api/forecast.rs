use std::fmt::Display;
use chrono::{DateTime, TimeZone, Timelike};

use super::BaseApi;
use crate::{Client, Query, Language, Forecast};


pub struct ForecastApi<'a, Tz: TimeZone> 
where
    Tz::Offset: Display
{
    client: &'a Client,
    query: Option<Query>,
    days: Option<u8>,
    dt: Option<DateTime<Tz>>,
    hour: bool,
    alerts: bool,
    aqi: bool,
    lang: Option<Language>
}

impl<'a, Tz: TimeZone> ForecastApi<'a, Tz>
where
    Tz::Offset: Display,
{
    /// Use [`crate::Client`]
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            query: None,
            days: None,
            dt: None,
            hour: false,
            alerts: false,
            aqi: false,
            lang: None
        }
    }

    /// Set up the query
    /// 
    /// Query parameter based on which data is sent back
    pub fn query(&mut self, query: Query) -> &mut Self {
        self.query = Some(query);
        self
    }

    /// Set up a days
    /// 
    /// Days parameter value ranges between 1 and 14. e.g: days=5
    /// 
    /// If no days parameter is provided then only today's weather is returned. 
    pub fn days(&mut self, days: u8) -> &mut Self {
        self.days = Some(days);
        self
    }

    /// Set up a datetime
    /// 
    /// `dt` should be between today and next 14 day 
    pub fn dt(&mut self, dt: DateTime<Tz>) -> &mut Self {
        self.dt = Some(dt);
        self
    }

    /// Set up use hour
    /// 
    /// Time is extracted from dt
    pub fn hour(&mut self, hour: bool) -> &mut Self {
        self.hour = hour;
        self
    }

    /// Set up alerts
    pub fn alerts(&mut self, alerts: bool) -> &mut Self {
        self.alerts = alerts;
        self
    }

    /// Set up Air Quality Data
    /// 
    /// Air quality data is not passed by default.
    pub fn aqi(&mut self, aqi: bool) -> &mut Self {
        self.aqi = aqi;
        self
    }

    /// Set up language 
    /// 
    /// `condition:text` field in API in the desired language
    pub fn lang(&mut self, lang: Language) -> &mut Self {
        self.lang = Some(lang);
        self
    }
}

impl<'a, Tz: TimeZone> BaseApi<'a> for ForecastApi<'a, Tz> 
where
    Tz::Offset: Display,
{
    type Model = Forecast;

    fn path(&self) -> &str {
        "forecast"
    }

    fn client(&self) -> &'a Client {
        self.client
    }

    fn params(&self) -> Vec<(&str, String)> {
        let query = self.query.as_ref().unwrap();
        let dt = self.dt.as_ref().unwrap();

        let mut params = vec![
            ("key", self.client.api_key.clone()), ("q", query.to_string()), ("dt", format!("{}", dt.format("%Y-%m-%d"))), 
            ("alerts", if self.alerts { "yes".to_string() } else { "no".to_string() }),
            ("aqi", if self.aqi { "yes".to_string() } else { "no".to_string() })
        ];
        
        if let Some(days) = self.days {
            params.push(("days", days.to_string()))
        }

        if self.hour {
            params.push(("hour", dt.hour().to_string()))
        }

        if let Some(lang) = &self.lang {
            params.push(("lang", lang.to_string()))
        }

        params
    }
}
