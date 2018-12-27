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
#[cfg(feature = "mocking")]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use test_utils::config::test_set_default_err;
    use test_utils::config::test_set_default_ok;

    #[test]
    fn config_type_set_default_when_ok() {
        test_set_default_ok::<ConfigEntry>("audio", default_map);
    }

    #[test]
    fn config_type_set_default_when_err() {
        test_set_default_err::<ConfigEntry>("audio", default_map);
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
