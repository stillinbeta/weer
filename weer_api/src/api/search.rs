use super::BaseApi;
use crate::{Client, Query, Location};

pub struct SearchApi<'a> {
    client: &'a Client,
    query: Option<Query>,
}

impl<'a> SearchApi<'a> {
    /// Use [`crate::Client`]
    pub fn new(client: &'a Client) -> Self {
        Self { 
            client, 
            query: None
        }
    }
    
    /// Set up the query
    /// 
    /// Query parameter based on which data is sent back
    pub fn query(mut self, query: Query) -> Self {
        self.query = Some(query);
        self
    }
}

impl<'a> BaseApi<'a> for SearchApi<'a> {
    type Model = Vec<Location>;

    fn path(&self) -> &str {
        "search"
    }

    fn client(&self) -> &'a Client {
        self.client
    }

    fn params(&self) -> Vec<(&str, String)> {
        let query = self.query.as_ref().unwrap();
        vec![("key", self.client.api_key.clone()), ("q", query.to_string())]
    }
}
