use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RenderConfig {
    pub(super) icons: Vec<String>,
    pub(super) template: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(super) device: String,
    #[serde(flatten)]
    pub(super) render: RenderConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut Config) -> Result<(), ConfigError> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "device"   => "intel_backlight",
                "icons"    => Vec::<String>::new(),
                "template" => "L {BL}%",
            ),
        )?;
        Ok(())
    }
}
