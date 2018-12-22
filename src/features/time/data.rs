use chrono;
use feature;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    format: String,
}

impl Data {
    pub(super) fn new(format: String) -> Self {
        Self {
            cache: String::new(),
            format,
        }
    }

    pub(super) fn update(&mut self, datetime: chrono::DateTime<chrono::Local>) {
        self.cache = format!("{}", datetime.format(&self.format));
    }
}

impl feature::Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}
