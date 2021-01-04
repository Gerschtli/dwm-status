use super::RenderConfig;
use crate::feature::Renderable;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    config: RenderConfig,
}

impl Data {
    #[allow(clippy::missing_const_for_fn)]
    pub(super) fn new(config: RenderConfig) -> Self {
        Self {
            cache: String::new(),
            config,
        }
    }

    pub(super) fn update(&mut self, value: u8) {
        if self.config.flags.len() > value as usize {
            self.cache = self
                .config
                .template
                .replace("{FLAG}", &self.config.flags[value as usize]);
        } else {
            panic!(
                "There's not enough flags in the configuration to display group {}",
                value
            );
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
    use super::*;
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;
    #[cfg(feature = "mocking")]
    use mocktopus::mocking::*;

    #[test]
    fn render_with_default() {
        let config = RenderConfig {
            flags: vec![],
            template: "TEMPLATE".to_owned(),
        };

        let object = Data::new(config);

        assert_that!(object.render(), is(equal_to("")));
    }

    #[test]
    #[should_panic]
    fn render_with_value_not_enough_flags() {
        let config = RenderConfig {
            flags: vec!["us".to_owned(), "ru".to_owned()],
            template: "TEMPLATE {FLAG}".to_owned(),
        };

        let mut object = Data::new(config);
        object.update(3);
    }

    #[test]
    fn render_with_value() {
        let config = RenderConfig {
            flags: vec!["us".to_owned(), "ru".to_owned()],
            template: "TEMPLATE {FLAG}".to_owned(),
        };

        let mut object = Data::new(config);
        object.update(1);

        assert_that!(object.render(), is(equal_to("TEMPLATE ru")));
    }
}
