use url::Url;
use ureq::Error;
use serde::{Serialize, Deserialize};
use std::{time::Duration, thread};

use crate::Client;

pub trait BaseApi<'a> 
where
    Self::Model: Serialize + for<'de> Deserialize<'de>
{
    type Model;

    fn path(&self) -> &str;
    fn params(&self) -> Vec<(&str, String)>;
    fn client(&self) -> &'a Client;

    fn call(&self) -> Result<Self::Model, ureq::Error> {
        let client = self.client();
        let url = Url::parse_with_params(
            format!(
                "http{}://api.weatherapi.com/v1/{}.json", 
                if client.https { "s" } else { "" }, 
                self.path()
            ).as_str(), 
            &self.params()
        )?;
        
        for _ in 1..2 {
            match client.agent.request_url("GET", &url).call() {
                Err(Error::Status(503, r)) | Err(Error::Status(429, r)) => {
                    let retry: Option<u64> = r.header("retry-after")
                        .and_then(|h| h.parse().ok());
                    let retry = retry.unwrap_or(5);
                    thread::sleep(Duration::from_secs(retry));
                }
                result => return Ok(result?.into_json()?),
            };
        }

        // Ran out of retries; try one last time and return whatever result we get.
        Ok(client.agent.request_url("GET", &url).call()?.into_json()?)
    }
}
