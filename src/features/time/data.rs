use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug)]
pub struct Time(pub DateTime<Local>);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}
