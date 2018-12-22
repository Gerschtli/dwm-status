use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

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
