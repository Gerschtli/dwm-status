use super::FEATURE_NAME;
use error::*;
use settings::ConfigType;
use wrapper::config;
use wrapper::config::Value;

#[derive(Clone, Debug, Deserialize)]
pub(super) struct RenderConfig {
    pub(super) icons: Vec<String>,
    pub(super) mute: String,
    pub(super) template: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) control: String,
    #[serde(flatten)]
    pub(super) render: RenderConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "control"  => "Master",
                "icons"    => Vec::<String>::new(),
                "mute"     => "MUTE",
                "template" => "S {VOL}%",
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;
    #[cfg(feature = "mocking")]
    use mocktopus::mocking::*;
    use std::collections::HashMap;

    #[cfg(feature = "mocking")]
    #[test]
    #[allow(unsafe_code)]
    fn config_type_set_default_when_ok() {
        let mut counter = 0;
        unsafe {
            config::Config::set_default.mock_raw(|_, key, value: HashMap<String, Value>| {
                counter += 1;

                let map = default_map();

                assert_that!(key, is(equal_to("audio")));
                assert_that!(value, is(equal_to(map)));

                MockResult::Return(Ok(()))
            });
        }

        let mut config = config::Config::new();

        assert_that!(ConfigEntry::set_default(&mut config), is(equal_to(Ok(()))));
        assert_that!(counter, is(equal_to(1)));
    }

    #[cfg(feature = "mocking")]
    #[test]
    #[allow(unsafe_code)]
    fn config_type_set_default_when_err() {
        let mut counter = 0;
        unsafe {
            config::Config::set_default.mock_raw(|_, key, value: HashMap<String, Value>| {
                counter += 1;

                let map = default_map();

                assert_that!(key, is(equal_to("audio")));
                assert_that!(value, is(equal_to(map)));

                MockResult::Return(Err(Error::new_custom("name", "description")))
            });
        }

        let mut config = config::Config::new();

        assert_that!(
            ConfigEntry::set_default(&mut config),
            is(equal_to(Err(Error::new_custom("name", "description"))))
        );
        assert_that!(counter, is(equal_to(1)));
    }

    fn default_map() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(String::from("control"), "Master".into());
        map.insert(String::from("icons"), Vec::<String>::new().into());
        map.insert(String::from("mute"), "MUTE".into());
        map.insert(String::from("template"), "S {VOL}%".into());

        map
    }
}
