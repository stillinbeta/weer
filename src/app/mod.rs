mod cli;
mod handler;

use clap::ArgMatches;
use std::{error::Error};
use serde::{Serialize, Deserialize};
use ureq::{Agent, AgentBuilder};
use weer_api::{Client, Language};


#[derive(Serialize, Deserialize)]
pub struct Config {
    lang: Option<String>,
    api_key: String
}

impl Default for Config {
    fn default() -> Self {
        Self { lang: None, api_key: "b406c89026bf4209b5511231222906".to_string() }
        
        // panic!("app not configured")
    }
}

impl Config {
    pub fn lang(&self) -> Option<Language> {
        if let Some(l) = &self.lang {
            Some(Language::new(l).unwrap())
        } else {
            None
        }
    }
}


pub struct App {
    pub matches: ArgMatches,
    pub cfg: Config,
    pub client: Client,
    pub agent: Agent
}

impl App {
    pub fn new() -> Self {
        let cmd = cli::build();
        let agent = AgentBuilder::new().build();
        let cfg: Config = confy::load("weer").unwrap();
        let client = Client::new(&cfg.api_key, true);

        Self {
            matches: cmd.get_matches(),
            cfg,
            client,
            agent
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        handler::matches_handler(&self)
    }
}
