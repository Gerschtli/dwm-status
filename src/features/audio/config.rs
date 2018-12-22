use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(crate) control: String,
    pub(crate) icons: Vec<String>,
    pub(crate) mute: String,
    pub(crate) template: String,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut Config) -> Result<(), ConfigError> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "control"  => "Master",
                "icons"    => Vec::<String>::new(),
                "mute"     => "MUTE",
                "template" => "S {VOL}%",
            ),
        )?;

        Ok(())
    }
}
