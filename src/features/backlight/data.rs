use crate::feature::Renderable;
use crate::settings::generate_status2d_data;
use crate::utils::icon_by_percentage;

use super::RenderConfig;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    config: RenderConfig,
}

impl Data {
    pub(super) const fn new(config: RenderConfig) -> Self {
        Self {
            cache: String::new(),
            config,
        }
    }

    pub(super) fn update(&mut self, value: u32) {
        let mut rendered = self.config.template.replace("{BL}", &format!("{}", value));

        if let Some(icon) = icon_by_percentage(&self.config.icons, value) {
            rendered = rendered.replace("{ICO}", icon);
        }

        self.cache = rendered;
    }

    pub(super) fn with_status2d(&mut self) {
        if let Some(status2d) = generate_status2d_data(&self.config.status2d) {
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
        let config = RenderConfig {
            icons: vec![],
            template: "TEMPLATE".to_owned(),
            status2d: vec![],
        };

        let object = Data::new(config);

        assert_that!(object.render(), is(equal_to("")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_volume() {
        let config = RenderConfig {
            icons: vec![],
            template: "TEMPLATE {BL} {ICO}".to_owned(),
            status2d: vec![],
        };

        icon_by_percentage.mock_safe(|icons, value: u32| {
            assert_that!(icons, empty());
            assert_that!(value, is(equal_to(10)));

            MockResult::Return(None)
        });

        let mut object = Data::new(config);

        object.update(10);

        assert_that!(object.render(), is(equal_to("TEMPLATE 10 {ICO}")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_volume_and_icon() {
        let config = RenderConfig {
            icons: vec!["ico1".to_owned(), "ico2".to_owned()],
            template: "TEMPLATE {BL} {ICO}".to_owned(),
            status2d: vec![],
        };

        icon_by_percentage.mock_safe(|icons, value: u32| {
            let expected_icons = vec!["ico1".to_owned(), "ico2".to_owned()];
            assert_that!(icons, contains(expected_icons).exactly());
            assert_that!(value, is(equal_to(10)));

            MockResult::Return(Some("ICON"))
        });

        let mut object = Data::new(config);

        object.update(10);

        assert_that!(object.render(), is(equal_to("TEMPLATE 10 ICON")));
    }
}
