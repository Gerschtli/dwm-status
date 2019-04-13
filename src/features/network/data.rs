use super::RenderConfig;
use super::PLACEHOLDER_ESSID;
use super::PLACEHOLDER_IPV4;
use super::PLACEHOLDER_IPV6;
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

    pub(super) fn update<T4, T6, E>(&mut self, ipv4: T4, ipv6: T6, essid: E)
    where
        T4: Into<Option<String>>,
        T6: Into<Option<String>>,
        E: Into<Option<String>>,
    {
        self.cache = self
            .config
            .template
            .replace(PLACEHOLDER_IPV4, &self.get_value(ipv4))
            .replace(PLACEHOLDER_IPV6, &self.get_value(ipv6))
            .replace(PLACEHOLDER_ESSID, &self.get_value(essid));
    }

    fn get_value<T: Into<Option<String>>>(&self, value: T) -> String {
        value.into().unwrap_or_else(|| self.config.no_value.clone())
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

    #[test]
    fn render_with_default() {
        let object = Data::new(RenderConfig {
            no_value: "--".to_owned(),
            template: "{IPv4} {IPv6} {ESSID}".to_owned(),
        });

        assert_that!(object.render(), is(equal_to("")));
    }

    #[test]
    fn render_with_update() {
        let mut object = Data::new(RenderConfig {
            no_value: "--".to_owned(),
            template: "{IPv4} {IPv6} {ESSID}".to_owned(),
        });

        object.update(
            "127.0.0.1".to_owned(),
            "fe::1".to_owned(),
            "WLAN".to_owned(),
        );

        assert_that!(object.render(), is(equal_to("127.0.0.1 fe::1 WLAN")));
    }

    #[test]
    fn render_with_update_and_missing_placeholder() {
        let mut object = Data::new(RenderConfig {
            no_value: "#".to_owned(),
            template: "{IPv4} // {ESSID}".to_owned(),
        });

        object.update("127.0.0.1".to_owned(), "fe::1".to_owned(), None);

        assert_that!(object.render(), is(equal_to("127.0.0.1 // #")));
    }

    #[test]
    fn render_with_update_and_none_values() {
        let mut object = Data::new(RenderConfig {
            no_value: "--".to_owned(),
            template: "{IPv4} {IPv6} {ESSID}".to_owned(),
        });

        object.update(None, None, None);

        assert_that!(object.render(), is(equal_to("-- -- --")));
    }
}
