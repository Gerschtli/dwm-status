use chrono;
use std::fmt;

#[derive(Debug)]
pub struct TimeData(pub chrono::DateTime<chrono::Local>);

impl fmt::Display for TimeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_display() {
        let date = chrono::Local.ymd(2014, 2, 1).and_hms(9, 0, 9);
        assert_eq!(format!("{}", TimeData(date)), "2014-02-01 09:00");
    }
}
