use super::FEATURE_NAME;
use error::*;
use settings::ConfigType;
use wrapper::config;
use wrapper::config::Value;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) format: String,
    pub(super) update_seconds: bool,
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
