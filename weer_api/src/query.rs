use std::fmt::{self, Display};
use std::net::IpAddr;


#[derive(Debug, PartialEq)]
pub enum Query {
    Coords(f32, f32),
    City(String),
    Ip(Option<IpAddr>)
}

impl Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Query::Coords(lat, long) => write!(f, "{lat},{long}"),
            Query::City(name) => write!(f, "{name}"),
            Query::Ip(Some(ip)) => write!(f, "{ip}"),
            Query::Ip(None) => write!(f, "auto:ip")
        }
    }
}