use serde_derive::Deserialize;

use crate::error::Result;
use crate::settings::ConfigType;
use crate::settings::Status2dEntry;
use crate::wrapper::config;
use crate::wrapper::config::Value;

use super::FEATURE_NAME;
use super::PLACEHOLDER_ESSID;
use super::PLACEHOLDER_IPV4;
use super::PLACEHOLDER_IPV6;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RenderConfig {
    pub(super) no_value: String,
    pub(super) template: String,
    pub(super) status2d: Vec<Status2dEntry>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(super) struct UpdateConfig {
    pub(super) show_essid: bool,
    pub(super) show_ipv4: bool,
    pub(super) show_ipv6: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    #[serde(flatten)]
    pub(super) render: RenderConfig,
    #[doc(hidden)]
    pub(super) update: UpdateConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "no_value" => "NA",
                "template" => "{IPv4} · {IPv6} · {ESSID}",
            ),
        )
    }

    fn set_values(config: &mut config::Config) -> Result<()> {
        let template = config.get_str(&format!("{}.template", FEATURE_NAME))?;

        set_update_config(config, &template, PLACEHOLDER_ESSID, "essid")?;
        set_update_config(config, &template, PLACEHOLDER_IPV4, "ipv4")?;
        set_update_config(config, &template, PLACEHOLDER_IPV6, "ipv6")?;

        Ok(())
    }
}

fn set_update_config(
    config: &mut config::Config,
    template: &str,
    placeholder: &str,
    key: &str,
) -> Result<()> {
    config.set(
        &format!("{}.update.show_{}", FEATURE_NAME, key),
        template.contains(placeholder),
    )
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
            test_set_default_ok::<ConfigEntry>("network", default_map);
        }

        #[test]
        fn when_err() {
            test_set_default_err::<ConfigEntry>("network", default_map);
        }

        fn default_map() -> HashMap<String, Value> {
            let mut map = HashMap::new();
            map.insert("no_value".to_owned(), "NA".into());
            map.insert("template".to_owned(), "{IPv4} · {IPv6} · {ESSID}".into());

            map
        }
    }

    mod config_type_set_values {
        use super::*;

        mod when_ok {
            use super::*;

            #[test]
            fn and_all_enabled() {
                test_config("{IPv4} · {IPv6} · {ESSID}", true, true, true);
            }

            #[test]
            fn and_essid_enabled() {
                test_config("IPv4} · IPv6} · {ESSID}", true, false, false);
            }

            #[test]
            fn and_ipv4_enabled() {
                test_config("{IPv4} · IPv6} · ESSID}", false, true, false);
            }

            #[test]
            fn and_ipv6_enabled() {
                test_config("IPv4} · {IPv6} · ESSID}", false, false, true);
            }

            #[test]
            fn and_nothing_enabled() {
                test_config("schubidu", false, false, false);
            }

            fn test_config(
                template: &'static str,
                show_essid: bool,
                show_ipv4: bool,
                show_ipv6: bool,
            ) {
                config::Config::get_str.mock_safe(move |_, key| {
                    assert_that!(key, is(equal_to("network.template")));
                    MockResult::Return(Ok(template.to_owned()))
                });

                let mut counter = 0;
                config::Config::set::<bool>.mock_safe(move |_, key, value| {
                    counter += 1;

                    match counter {
                        1 => {
                            assert_that!(key, is(equal_to("network.update.show_essid")));
                            assert_that!(value, is(equal_to(show_essid)));
                            MockResult::Return(Ok(()))
                        },
                        2 => {
                            assert_that!(key, is(equal_to("network.update.show_ipv4")));
                            assert_that!(value, is(equal_to(show_ipv4)));
                            MockResult::Return(Ok(()))
                        },
                        3 => {
                            assert_that!(key, is(equal_to("network.update.show_ipv6")));
                            assert_that!(value, is(equal_to(show_ipv6)));
                            MockResult::Return(Ok(()))
                        },
                        _ => panic!("set called to often: {} times", counter),
                    }
                });

                let mut config = config::Config::new();

                assert_that!(ConfigEntry::set_values(&mut config), is(equal_to(Ok(()))));
            }
        }

        mod when_err {
            use super::*;

            #[test]
            fn in_get_str() {
                config::Config::get_str.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("network.template")));
                    MockResult::Return(Err(Error::new_custom("name", "description")))
                });

                let mut config = config::Config::new();

                assert_that!(
                    ConfigEntry::set_values(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }

            #[test]
            fn in_first_set() {
                config::Config::get_str.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("network.template")));
                    MockResult::Return(Ok("template".to_owned()))
                });

                let mut counter = 0;
                config::Config::set::<bool>.mock_safe(move |_, key, value| {
                    counter += 1;

                    match counter {
                        1 => {
                            assert_that!(key, is(equal_to("network.update.show_essid")));
                            assert_that!(value, is(equal_to(false)));
                            MockResult::Return(Err(Error::new_custom("name", "description")))
                        },
                        _ => panic!("set called to often: {} times", counter),
                    }
                });

                let mut config = config::Config::new();

                assert_that!(
                    ConfigEntry::set_values(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }

            #[test]
            fn in_second_set() {
                config::Config::get_str.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("network.template")));
                    MockResult::Return(Ok("template".to_owned()))
                });

                let mut counter = 0;
                config::Config::set::<bool>.mock_safe(move |_, key, value| {
                    counter += 1;

                    match counter {
                        1 => {
                            assert_that!(key, is(equal_to("network.update.show_essid")));
                            assert_that!(value, is(equal_to(false)));
                            MockResult::Return(Ok(()))
                        },
                        2 => {
                            assert_that!(key, is(equal_to("network.update.show_ipv4")));
                            assert_that!(value, is(equal_to(false)));
                            MockResult::Return(Err(Error::new_custom("name", "description")))
                        },
                        _ => panic!("set called to often: {} times", counter),
                    }
                });

                let mut config = config::Config::new();

                assert_that!(
                    ConfigEntry::set_values(&mut config),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }

            #[test]
            fn in_third_set() {
                config::Config::get_str.mock_safe(|_, key| {
                    assert_that!(key, is(equal_to("network.template")));
                    MockResult::Return(Ok("template".to_owned()))
                });

                let mut counter = 0;
                config::Config::set::<bool>.mock_safe(move |_, key, value| {
                    counter += 1;

                    match counter {
                        1 => {
                            assert_that!(key, is(equal_to("network.update.show_essid")));
                            assert_that!(value, is(equal_to(false)));
                            MockResult::Return(Ok(()))
                        },
                        2 => {
                            assert_that!(key, is(equal_to("network.update.show_ipv4")));
                            assert_that!(value, is(equal_to(false)));
                            MockResult::Return(Ok(()))
                        },
                        3 => {
                            assert_that!(key, is(equal_to("network.update.show_ipv6")));
                            assert_that!(value, is(equal_to(false)));
                            MockResult::Return(Err(Error::new_custom("name", "description")))
                        },
                        _ => panic!("set called to often: {} times", counter),
                    }
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
