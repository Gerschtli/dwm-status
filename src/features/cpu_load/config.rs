use serde_derive::Deserialize;

use crate::error::Result;
use crate::settings::{ConfigType, Status2dEntry};
use crate::wrapper::config;
use crate::wrapper::config::Value;

use super::FEATURE_NAME;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) template: String,
    pub(super) update_interval: u64,
    pub(super) status2d: Vec<Status2dEntry>,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "template"        => "{CL1} {CL5} {CL15}",
                "update_interval" => 20,
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
        test_set_default_ok::<ConfigEntry>("cpu_load", default_map);
    }

    #[test]
    fn config_type_set_default_when_err() {
        test_set_default_err::<ConfigEntry>("cpu_load", default_map);
    }

    fn default_map() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("template".to_owned(), "{CL1} {CL5} {CL15}".into());
        map.insert("update_interval".to_owned(), 20.into());

        map
    }
}
