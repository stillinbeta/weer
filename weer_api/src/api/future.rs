use std::fmt::Display;
use chrono::{DateTime, TimeZone};

use super::BaseApi;
use crate::{Client, Query, Language, Future};

pub struct FutureApi<'a, Tz: TimeZone> 
where
    Tz::Offset: Display
{
    client: &'a Client,
    query: Option<Query>,
    dt: Option<DateTime<Tz>>,
    lang: Option<Language>
}

impl<'a, Tz: TimeZone> FutureApi<'a, Tz> 
where
    Tz::Offset: Display
{
    /// Use [`crate::Client`]
    pub fn new(client: &'a Client) -> Self {
        Self { 
            client, 
            query: None, 
            dt: None,
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
    /// `dt` should be between 14 days and 300 days from today in the future
    pub fn dt(&mut self, dt: DateTime<Tz>) -> &mut Self {
        self.dt = Some(dt);
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

impl<'a, Tz: TimeZone> BaseApi<'a> for FutureApi<'a, Tz> 
where
    Tz::Offset: Display 
{
    type Model = Future;

    fn path(&self) -> &str {
        "future"
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

        if let Some(lang) = &self.lang {
            params.push(("lang", lang.to_string()))
        }

        params
    }
}
