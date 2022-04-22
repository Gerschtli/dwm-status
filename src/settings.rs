use log::warn;
use serde_derive::Deserialize;

use crate::error::Result;
use crate::features;
use crate::wrapper::config;

pub(crate) trait ConfigType {
    fn set_default(_: &mut config::Config) -> Result<()>;

    fn set_values(_: &mut config::Config) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct General {
    #[allow(dead_code)]
    pub(crate) debug: Option<bool>,
    pub(crate) order: Vec<String>,
    pub(crate) separator: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Status2dEntry {
    pub(crate) style: Option<String>,
    pub(crate) value: Option<String>,
}

impl Status2dEntry {
    pub(crate) fn generate(&self) -> Option<String> {
        match (&self.style, &self.value) {
            (Some(x), Some(y)) => Some(format!("^{}{}^", x, y)),
            (..) => None,
        }
    }
}

pub(crate) fn generate_status2d_data(entries: &[Status2dEntry]) -> Option<String> {
    if entries.is_empty() {
        return None;
    }

    let res = entries
        .iter()
        .map(|entry| entry.generate())
        .filter_map(|s| s)
        .collect::<Vec<String>>()
        .join("");

    if res.is_empty() {
        return None;
    }

    Some(res)
}

impl ConfigType for General {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default("debug", None::<bool>)?;
        config.set_default("order", Vec::<String>::new())?;
        config.set_default("separator", " / ")?;

        Ok(())
    }

    fn set_values(config: &mut config::Config) -> Result<()> {
        let debug = config.get_bool_option("debug")?;

        if debug.is_some() {
            warn!(
                "Config option 'debug' is deprecated and will be removed in 2.0.0. Log level is \
                 set to info by default."
            );
        }

        Ok(())
    }
}

macro_rules! settings {
    ( $( $mod:ident ),* ) => {
        #[derive(Clone, Debug, Deserialize)]
        pub(crate) struct Settings {
            #[serde(flatten)]
            pub(crate) general: General,
            $(
                pub(crate) $mod: features::$mod::ConfigEntry,
            )*
        }

        impl Settings {
            pub(crate) fn init(config_path: &str) -> Result<Self> {
                let mut config = config::Config::new();

                General::set_default(&mut config)?;
                $(
                    features::$mod::ConfigEntry::set_default(&mut config)?;
                )*

                config.set_path(config_path)?;

                General::set_values(&mut config)?;
                $(
                    features::$mod::ConfigEntry::set_values(&mut config)?;
                )*

                config.try_into()
            }
        }
    }
}

settings!(audio, backlight, battery, cpu_load, network, time);

#[cfg(test)]
#[cfg(feature = "mocking")]
mod tests {
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;
    use mocktopus::mocking::*;

    use crate::error::Error;

    use super::*;

    mod general_config_type {
        use super::*;

        mod set_default {
            use super::*;

            #[test]
            fn when_first_call_failed() {
                let mut counter_first = 0;
                unsafe {
                    config::Config::set_default::<Option<bool>>.mock_raw(|_, key, value| {
                        counter_first += 1;

                        assert_that!(key, is(equal_to("debug")));
                        assert_that!(value, is(none()));

                        MockResult::Return(Err(Error::new_custom("name", "description")))
                    });
                }

                let mut config = config::Config::new();

                assert_that!(
                    General::set_default(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
                assert_that!(counter_first, is(equal_to(1)));
            }

            #[test]
            fn when_second_call_failed() {
                let mut counter_first = 0;
                unsafe {
                    config::Config::set_default::<Option<bool>>.mock_raw(|_, key, value| {
                        counter_first += 1;

                        assert_that!(key, is(equal_to("debug")));
                        assert_that!(value, is(none()));

                        MockResult::Return(Ok(()))
                    });
                }

                let mut counter_second = 0;
                unsafe {
                    config::Config::set_default::<Vec<String>>.mock_raw(|_, key, value| {
                        counter_second += 1;

                        assert_that!(key, is(equal_to("order")));
                        assert_that!(&value, is(empty()));

                        MockResult::Return(Err(Error::new_custom("name", "description")))
                    });
                }

                let mut config = config::Config::new();

                assert_that!(
                    General::set_default(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
                assert_that!(counter_first, is(equal_to(1)));
                assert_that!(counter_second, is(equal_to(1)));
            }

            #[test]
            fn when_third_call_failed() {
                let mut counter_first = 0;
                unsafe {
                    config::Config::set_default::<Option<bool>>.mock_raw(|_, key, value| {
                        counter_first += 1;

                        assert_that!(key, is(equal_to("debug")));
                        assert_that!(value, is(none()));

                        MockResult::Return(Ok(()))
                    });
                }

                let mut counter_second = 0;
                unsafe {
                    config::Config::set_default::<Vec<String>>.mock_raw(|_, key, value| {
                        counter_second += 1;

                        assert_that!(key, is(equal_to("order")));
                        assert_that!(&value, is(empty()));

                        MockResult::Return(Ok(()))
                    });
                }

                let mut counter_third = 0;
                unsafe {
                    config::Config::set_default::<&str>.mock_raw(|_, key, value| {
                        counter_third += 1;

                        assert_that!(key, is(equal_to("separator")));
                        assert_that!(value, is(equal_to(" / ")));

                        MockResult::Return(Err(Error::new_custom("name", "description")))
                    });
                }

                let mut config = config::Config::new();

                assert_that!(
                    General::set_default(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
                assert_that!(counter_first, is(equal_to(1)));
                assert_that!(counter_second, is(equal_to(1)));
                assert_that!(counter_third, is(equal_to(1)));
            }

            #[test]
            fn when_all_calls_succeed() {
                let mut counter_first = 0;
                unsafe {
                    config::Config::set_default::<Option<bool>>.mock_raw(|_, key, value| {
                        counter_first += 1;

                        assert_that!(key, is(equal_to("debug")));
                        assert_that!(value, is(none()));

                        MockResult::Return(Ok(()))
                    });
                }

                let mut counter_second = 0;
                unsafe {
                    config::Config::set_default::<Vec<String>>.mock_raw(|_, key, value| {
                        counter_second += 1;

                        assert_that!(key, is(equal_to("order")));
                        assert_that!(&value, is(empty()));

                        MockResult::Return(Ok(()))
                    });
                }

                let mut counter_third = 0;
                unsafe {
                    config::Config::set_default::<&str>.mock_raw(|_, key, value| {
                        counter_third += 1;

                        assert_that!(key, is(equal_to("separator")));
                        assert_that!(value, is(equal_to(" / ")));

                        MockResult::Return(Ok(()))
                    });
                }

                let mut config = config::Config::new();

                assert_that!(General::set_default(&mut config), is(equal_to(Ok(()))));
                assert_that!(counter_first, is(equal_to(1)));
                assert_that!(counter_second, is(equal_to(1)));
                assert_that!(counter_third, is(equal_to(1)));
            }
        }

        mod set_values {
            use crate::test_utils::log::Level;
            use crate::test_utils::log::LoggerContext;

            use super::*;

            #[test]
            fn when_get_option_failed() {
                let _ = LoggerContext::new();

                config::Config::get_bool_option.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("debug")));

                    MockResult::Return(Err(Error::new_custom("name", "description")))
                });

                let mut config = config::Config::new();

                assert_that!(
                    General::set_values(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }

            mod when_is_some {
                use super::*;

                #[test]
                fn with_value_true() {
                    with_value(true);
                }

                #[test]
                fn with_value_false() {
                    with_value(false);
                }

                fn with_value(value: bool) {
                    let logger_context = LoggerContext::new();

                    config::Config::get_bool_option.mock_safe(move |_, key| {
                        assert_that!(key, is(equal_to("debug")));

                        MockResult::Return(Ok(Some(value)))
                    });

                    let mut config = config::Config::new();

                    assert_that!(General::set_values(&mut config), is(equal_to(Ok(()))));

                    logger_context.assert_entry(
                        Level::Warn,
                        "Config option 'debug' is deprecated and will be removed in 2.0.0. Log \
                         level is set to info by default.",
                    );
                }
            }

            #[test]
            fn when_is_none() {
                let _ = LoggerContext::new();

                config::Config::get_bool_option.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("debug")));

                    MockResult::Return(Ok(None))
                });

                let mut config = config::Config::new();

                assert_that!(General::set_values(&mut config), is(equal_to(Ok(()))));
            }
        }
    }
}
