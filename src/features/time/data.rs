use feature::Renderable;
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

impl Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;
    #[cfg(feature = "mocking")]
    use mocktopus::mocking::*;

    #[test]
    fn render_with_default() {
        let object = Data::new(String::from("format"));

        assert_that!(object.render(), is(equal_to("")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_update() {
        let mut object = Data::new(String::from("format"));

        date_time::DateTime::format.mock_safe(|_, format| {
            assert_that!(format, is(equal_to("format")));

            MockResult::Return(String::from("formatted date time"))
        });

        object.update(date_time::DateTime::now());

        assert_that!(object.render(), is(equal_to("formatted date time")));
    }
}
