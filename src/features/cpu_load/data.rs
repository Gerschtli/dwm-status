use crate::{
    feature::Renderable,
    settings::{generate_status2d_data, Status2dEntry},
};

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    template: String,
    status2d: Vec<Status2dEntry>,
}

impl Data {
    pub(super) const fn new(template: String, status2d: Vec<Status2dEntry>) -> Self {
        Self {
            cache: String::new(),
            template,
            status2d,
        }
    }

    pub(super) fn update(&mut self, one: f32, five: f32, fifteen: f32) {
        self.cache = self
            .template
            .replace("{CL1}", &format!("{:.2}", one))
            .replace("{CL5}", &format!("{:.2}", five))
            .replace("{CL15}", &format!("{:.2}", fifteen));
    }

    pub(super) fn with_status2d(&mut self) {
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

    use super::*;

    #[test]
    fn render_with_default() {
        let object = Data::new("{CL1} {CL5} {CL15}".to_owned(), vec![]);

        assert_that!(object.render(), is(equal_to("")));
    }

    #[test]
    fn render_with_update() {
        let mut object = Data::new("{CL1} {CL5} {CL15}".to_owned(), vec![]);

        object.update(20.1234, 0.005, 5.3);

        assert_that!(object.render(), is(equal_to("20.12 0.00 5.30")));
    }

    #[test]
    fn render_with_update_and_missing_placeholder() {
        let mut object = Data::new("{CL1} - {CL15}".to_owned(), vec![]);

        object.update(20.1234, 0.005, 5.3);

        assert_that!(object.render(), is(equal_to("20.12 - 5.30")));
    }
}
