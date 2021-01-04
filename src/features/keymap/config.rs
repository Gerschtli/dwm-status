use super::FEATURE_NAME;
use crate::error::*;
use crate::settings::ConfigType;
use crate::wrapper::config;
use crate::wrapper::config::Value;
use serde_derive::*;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RenderConfig {
    pub(super) flags: Vec<String>,
    pub(super) template: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    #[serde(flatten)]
    pub(super) render: RenderConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut config::Config) -> Result<()> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "flags"    => Vec::<String>::new(),
                "template" => "{FLAG}",
            ),
        )
    }
}

#[cfg(test)]
#[cfg(feature = "mocking")]
mod tests {
    use super::*;
    use crate::test_utils::config::test_set_default_err;
    use crate::test_utils::config::test_set_default_ok;
    use std::collections::HashMap;

    #[test]
    fn config_type_set_default_when_ok() {
        test_set_default_ok::<ConfigEntry>("keymap", default_map);
    }

    #[test]
    fn config_type_set_default_when_err() {
        test_set_default_err::<ConfigEntry>("keymap", default_map);
    }

    fn default_map() -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("flags".to_owned(), Vec::<String>::new().into());
        map.insert("template".to_owned(), "{FLAG}".into());

        map
    }
}
