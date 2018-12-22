use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(crate) format: String,
    pub(crate) update_seconds: bool,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut Config) -> Result<(), ConfigError> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "format" => "%Y-%m-%d %H:%M",
                "update_seconds" => false,
            ),
        )?;
        Ok(())
    }

    fn set_values(config: &mut Config) -> Result<(), ConfigError> {
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
