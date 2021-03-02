use crate::feature::Renderable;
use crate::wrapper::date_time;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    format: String,
}

impl Data {
    pub(super) const fn new(format: String) -> Self {
        Self {
            cache: String::new(),
            format,
        }
    }

    pub(super) fn update(&mut self, date_time: &date_time::DateTime) {
        self.cache = date_time.format(&self.format);
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
        let object = Data::new("format".to_owned());

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
