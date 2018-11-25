use chrono;
use feature;
use settings;

#[derive(Debug)]
pub struct TimeData(pub chrono::DateTime<chrono::Local>);

impl feature::Renderable for TimeData {
    fn render(&self, settings: &settings::Settings) -> String {
        format!("{}", self.0.format(&settings.time.format))
    }
}

/* temporarily disabled because missing mock possibilty in tests
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use feature::Renderable;

    #[test]
    fn test_display() {
        let date = chrono::Local.ymd(2014, 2, 1).and_hms(9, 0, 9);
        assert_eq!(
            TimeData {
                format: String::from("%Y-%m-%d ---"),
                time: date,
            }
            .render(),
            "2014-02-01 ---"
        );
        assert_eq!(
            TimeData {
                format: String::from("%Y-%m-%d %H:%M:%S"),
                time: date,
            }
            .render(),
            "2014-02-01 09:00:09"
        );
    }
}
*/
