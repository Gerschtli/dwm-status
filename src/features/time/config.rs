use serde_derive::Deserialize;

use crate::error::Result;
use crate::settings::{ConfigType, Status2dEntry};
use crate::wrapper::config;
use crate::wrapper::config::Value;

use super::FEATURE_NAME;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) format: String,
    pub(super) update_seconds: bool,
    pub(super) status2d: Vec<Status2dEntry>,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "format"         => "%Y-%m-%d %H:%M",
                "update_seconds" => false,
            ),
        )
    }

    fn set_values(config: &mut config::Config) -> Result<()> {
        // dynamically set time.update_seconds
        let time_format = config
            .get_str(&format!("{}.format", FEATURE_NAME))?
            .replace("%%", "");

        if ["%f", "%r", "%S", "%s", "%T"]
            .iter()
            .any(|specifier| time_format.contains(specifier))
        {
            config.set_default(&format!("{}.update_seconds", FEATURE_NAME), true)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "mocking")]
mod tests {
    use std::collections::HashMap;

    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;
    use mocktopus::mocking::*;

    use crate::error::Error;
    use crate::test_utils::config::test_set_default_err;
    use crate::test_utils::config::test_set_default_ok;

    use super::*;

    mod config_type_set_default {
        use super::*;

        #[test]
        fn when_ok() {
            test_set_default_ok::<ConfigEntry>("time", default_map);
        }

        #[test]
        fn when_err() {
            test_set_default_err::<ConfigEntry>("time", default_map);
        }

        fn default_map() -> HashMap<String, Value> {
            let mut map = HashMap::new();
            map.insert("format".to_owned(), "%Y-%m-%d %H:%M".into());
            map.insert("update_seconds".to_owned(), false.into());

            map
        }
    }

    mod config_type_set_values {
        use super::*;

        mod when_all_ok {
            use super::*;

            #[test]
            fn and_matching_placeholder_lower_f() {
                test_builder("%f", true);
            }

            #[test]
            fn and_matching_placeholder_lower_r() {
                test_builder("%r", true);
            }

            #[test]
            fn and_matching_placeholder_upper_s() {
                test_builder("%S", true);
            }

            #[test]
            fn and_matching_placeholder_lower_s() {
                test_builder("%s", true);
            }

            #[test]
            fn and_matching_placeholder_upper_t() {
                test_builder("%T", true);
            }

            #[test]
            fn and_not_matching_escaped_percent() {
                test_builder("%%s %%f", false);
            }

            #[test]
            fn and_not_matching_any_wrong_placeholder() {
                test_builder("%Y-%m-%d %H:%M", false);
            }

            fn test_builder(format: &'static str, is_match: bool) {
                config::Config::get_str.mock_safe(move |_, key| {
                    assert_that!(key, is(equal_to("time.format")));
                    MockResult::Return(Ok(format.to_owned()))
                });

                let mut counter = 0;
                unsafe {
                    config::Config::set_default.mock_raw(|_, key, value: bool| {
                        counter += 1;
                        assert_that!(key, is(equal_to("time.update_seconds")));
                        assert_that!(value, is(equal_to(true)));
                        MockResult::Return(Ok(()))
                    });
                }

                let mut config = config::Config::new();

                assert_that!(ConfigEntry::set_values(&mut config), is(equal_to(Ok(()))));
                assert_that!(counter, is(equal_to(if is_match { 1 } else { 0 })));
            }
        }

        mod when_err {
            use super::*;

            #[test]
            fn in_get_str() {
                config::Config::get_str.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("time.format")));
                    MockResult::Return(Err(Error::new_custom("name", "description")))
                });

                let mut config = config::Config::new();

                assert_that!(
                    ConfigEntry::set_values(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }

            #[test]
            fn in_set_default() {
                config::Config::get_str.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("time.format")));
                    MockResult::Return(Ok("%s".to_owned()))
                });
                config::Config::set_default.mock_safe(|_, key, value: bool| {
                    assert_that!(key, is(equal_to("time.update_seconds")));
                    assert_that!(value, is(equal_to(true)));
                    MockResult::Return(Err(Error::new_custom("name", "description")))
                });

                let mut config = config::Config::new();

                assert_that!(
                    ConfigEntry::set_values(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }
        }
    }
}
