use super::RenderConfig;
use feature::Renderable;
use utils::icon_by_percentage;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    config: RenderConfig,
}

impl Data {
    pub(super) fn new(config: RenderConfig) -> Self {
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
        let config = RenderConfig {
            icons: vec![],
            template: String::from("TEMPLATE"),
        };

        let object = Data::new(config);

        assert_that!(object.render(), is(equal_to("")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_volume() {
        let config = RenderConfig {
            icons: vec![],
            template: String::from("TEMPLATE {BL} {ICO}"),
        };

        icon_by_percentage.mock_safe(|icons, value| {
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
            icons: vec![String::from("ico1"), String::from("ico2")],
            template: String::from("TEMPLATE {BL} {ICO}"),
        };

        icon_by_percentage.mock_safe(|icons, value| {
            let expected_icons = vec![String::from("ico1"), String::from("ico2")];
            assert_that!(icons, contains(expected_icons).exactly());
            assert_that!(value, is(equal_to(10)));

            MockResult::Return(Some("ICON"))
        });

        let mut object = Data::new(config);

        object.update(10);

        assert_that!(object.render(), is(equal_to("TEMPLATE 10 ICON")));
    }
}
