use crate::feature::Renderable;
use crate::settings::{generate_status2d_data, Status2dEntry};
use crate::wrapper::date_time;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    format: String,
    status2d: Vec<Status2dEntry>,
}

impl Data {
    pub(super) const fn new(format: String, status2d: Vec<Status2dEntry>) -> Self {
        Self {
            cache: String::new(),
            format,
            status2d,
        }
    }

    pub(super) fn update(&mut self, date_time: &date_time::DateTime) {
        self.cache = date_time.format(&self.format);

        if let Some(status2d) = generate_status2d_data(&self.status2d) {
            self.cache = format!("{}{}", &status2d, &self.cache);
        }
    }
}

impl Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;
    #[cfg(feature = "mocking")]
    use mocktopus::mocking::*;

    use super::*;

    #[test]
    fn render_with_default() {
        let object = Data::new("format".to_owned(), vec![]);

        assert_that!(object.render(), is(equal_to("")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_update() {
        let mut object = Data::new("format".to_owned());

        date_time::DateTime::format.mock_safe(|_, format| {
            assert_that!(format, is(equal_to("format")));

            MockResult::Return("formatted date time".to_owned())
        });

        object.update(&date_time::DateTime::now());

        assert_that!(object.render(), is(equal_to("formatted date time")));
    }
}
