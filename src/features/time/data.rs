use feature;
use wrapper::date_time;

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

    pub(super) fn update(&mut self, date_time: date_time::DateTime) {
        self.cache = date_time.format(&self.format);
    }
}

impl feature::Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}
