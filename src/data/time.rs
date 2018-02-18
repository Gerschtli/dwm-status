use chrono::{DateTime, Local};
use std::fmt;

use super::Init;

#[derive(Debug)]
pub struct Time(DateTime<Local>);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}

impl Init for Time {
    fn init() -> Self {
        Time(Local::now())
    }
}
