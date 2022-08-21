use std::fmt::Display;
use chrono::{DateTime, TimeZone, Timelike};

use super::BaseApi;
use crate::{Client, Query, Language, History};


pub struct HistoryApi<'a, Tz: TimeZone> 
where
    Tz::Offset: Display
{
    client: &'a Client,
    query: Option<Query>,
    dt: Option<DateTime<Tz>>,
    end_dt: Option<DateTime<Tz>>,
    hour: bool,
    lang: Option<Language>
}

impl<'a, Tz: TimeZone> HistoryApi<'a, Tz>
where
    Tz::Offset: Display,
{
    /// Use [`crate::Client`]
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            query: None,
            dt: None,
            end_dt: None,
            hour: false,
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

    /// Set up a datetime
    /// 
    /// `dt` should be on or after 1st Jan, 2010
    pub fn dt(&mut self, dt: DateTime<Tz>) -> &mut Self {
        self.dt = Some(dt);
        self
    }

    /// Set up a end datetime
    /// 
    /// `end_dt` should be on or after 1st Jan, 2010 and should be greater than `dt` parameter and difference should not be more than 30 days between the two dates.
    pub fn end_dt(&mut self, end_dt: DateTime<Tz>) -> &mut Self {
        self.end_dt = Some(end_dt);
        self
    }

    /// Set up use hour
    /// 
    /// Time is extracted from dt
    pub fn hour(&mut self) -> &mut Self {
        self.hour = true;
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

impl<'a, Tz: TimeZone> BaseApi<'a> for HistoryApi<'a, Tz> 
where
    Tz::Offset: Display,
{
    type Model = History;

    fn path(&self) -> &str {
        "history"
    }

    fn client(&self) -> &'a Client {
        self.client
    }

    fn params(&self) -> Vec<(&str, String)> {
        let query = self.query.as_ref().unwrap();
        let dt = self.dt.as_ref().unwrap();

        let mut params = vec![
            ("key", self.client.api_key.clone()), ("q", query.to_string()), 
            ("dt", format!("{}", dt.format("%Y-%m-%d"))), 
        ];

        if let Some(end_dt) = &self.end_dt {
            params.push(("end_dt", format!("{}", end_dt.format("%Y-%m-%d"))))
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
