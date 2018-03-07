use chrono;
use std::fmt;

#[derive(Debug)]
pub struct TimeData(pub chrono::DateTime<chrono::Local>);

impl fmt::Display for TimeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}
