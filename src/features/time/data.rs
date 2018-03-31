use chrono;
use feature;

#[derive(Debug)]
pub struct TimeData(pub chrono::DateTime<chrono::Local>);

impl feature::Renderable for TimeData {
    fn render(&self) -> String {
        format!("{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use feature::Renderable;

    #[test]
    fn test_display() {
        let date = chrono::Local.ymd(2014, 2, 1).and_hms(9, 0, 9);
        assert_eq!(TimeData(date).render(), "2014-02-01 09:00");
    }
}
