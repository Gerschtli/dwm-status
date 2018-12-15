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
