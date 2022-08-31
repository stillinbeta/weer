use super::BaseApi;
use crate::{Client, Query, Language, Realtime};

pub struct RealtimeApi<'a> {
    client: &'a Client,
    query: Option<Query>,
    aqi: bool,
    lang: Option<Language>
}

impl<'a> RealtimeApi<'a> {
    /// Use [`crate::Client`]
    pub fn new(client: &'a Client) -> Self {
        Self { 
            client, 
            query: None, 
            aqi: false, 
            lang: None 
        }
    }
    
    /// Set up the query
    /// 
    /// Query parameter based on which data is sent back
    pub fn query(mut self, query: Query) -> Self {
        self.query = Some(query);
        self
    }

    /// Set up Air Quality Data
    /// 
    /// Air quality data is not passed by default.
    pub fn aqi(mut self) -> Self {
        self.aqi = true;
        self
    }

    /// Set up language 
    /// 
    /// `condition:text` field in API in the desired language
    pub fn lang(mut self, lang: Language) -> Self {
        self.lang = Some(lang);
        self
    }
}

impl<'a> BaseApi<'a> for RealtimeApi<'a> {
    type Model = Realtime;

    fn path(&self) -> &str {
        "current"
    }

    fn client(&self) -> &'a Client {
        self.client
    }

    fn params(&self) -> Vec<(&str, String)> {
        let query = self.query.as_ref().unwrap();
        let mut params = vec![
            ("key", self.client.api_key.clone()), ("q", query.to_string()),
            ("aqi", if self.aqi { "yes".to_string() } else { "no".to_string() })
        ];

        if let Some(lang) = &self.lang {
            params.push(("lang", lang.to_string()))
        }

        params
    }
}
