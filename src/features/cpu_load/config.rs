use super::FEATURE_NAME;
use error::*;
use settings::ConfigType;
use wrapper::config;
use wrapper::config::Value;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) template: String,
    pub(super) update_interval: u64,
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
    use super::*;
    use std::collections::HashMap;
    use test_utils::config::test_set_default_err;
    use test_utils::config::test_set_default_ok;

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
        map.insert(String::from("template"), "{CL1} {CL5} {CL15}".into());
        map.insert(String::from("update_interval"), 20.into());

        map
    }
}
