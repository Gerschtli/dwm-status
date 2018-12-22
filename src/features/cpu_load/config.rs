use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) template: String,
    pub(super) update_interval: u64,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut Config) -> Result<(), ConfigError> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "template"        => "{CL1} {CL5} {CL15}",
                "update_interval" => 20,
            ),
        )?;
        Ok(())
    }
}
