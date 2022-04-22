use serde_derive::Deserialize;

use crate::error::Result;
use crate::settings::{ConfigType, Status2dEntry};
use crate::wrapper::config;
use crate::wrapper::config::Value;

use super::FEATURE_NAME;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct NotifierConfig {
    pub(super) enable_notifier: bool,
    pub(super) notifier_critical: u64,
    pub(super) notifier_levels: Vec<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RenderConfig {
    pub(super) charging: String,
    pub(super) discharging: String,
    pub(super) icons: Vec<String>,
    pub(super) no_battery: String,
    pub(super) separator: String,
    pub(super) status2d: Vec<Status2dEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    #[serde(flatten)]
    pub(super) notifier: NotifierConfig,
    #[serde(flatten)]
    pub(super) render: RenderConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "charging"          => "▲",
                "discharging"       => "▼",
                "enable_notifier"   => true,
                "icons"             => Vec::<String>::new(),
                "no_battery"        => "NO BATT",
                "notifier_critical" => 10,
                "notifier_levels"   => vec![2, 5, 10, 15, 20],
                "separator"         => " · ",
            ),
        )
    }
}

#[cfg(test)]
#[cfg(feature = "mocking")]
mod tests {
    use std::collections::HashMap;

    use crate::test_utils::config::test_set_default_err;
    use crate::test_utils::config::test_set_default_ok;

    use super::*;

    #[test]
    fn config_type_set_default_when_ok() {
        test_set_default_ok::<ConfigEntry>("battery", default_map);
    }

    #[test]
    fn config_type_set_default_when_err() {
        test_set_default_err::<ConfigEntry>("battery", default_map);
    }

    fn default_map() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("charging".to_owned(), "▲".into());
        map.insert("discharging".to_owned(), "▼".into());
        map.insert("enable_notifier".to_owned(), true.into());
        map.insert("icons".to_owned(), Vec::<String>::new().into());
        map.insert("no_battery".to_owned(), "NO BATT".into());
        map.insert("notifier_critical".to_owned(), 10.into());
        map.insert("notifier_levels".to_owned(), vec![2, 5, 10, 15, 20].into());
        map.insert("separator".to_owned(), " · ".into());

        map
    }
}
