use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct NotifierConfig {
    pub(super) enable_notifier: bool,
    pub(super) notifier_critical: u32,
    pub(super) notifier_levels: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RenderConfig {
    pub(super) charging: String,
    pub(super) discharging: String,
    pub(super) icons: Vec<String>,
    pub(super) no_battery: String,
    pub(super) separator: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    #[serde(flatten)]
    pub(super) notifier: NotifierConfig,
    #[serde(flatten)]
    pub(super) render: RenderConfig,
}

impl ConfigType for ConfigEntry {
    fn set_default(config: &mut Config) -> Result<(), ConfigError> {
        config.set_default(
            FEATURE_NAME,
            map!(
                "charging"          => "▲",
                "discharging"       => "▼",
                "enable_notifier"   => true,
                "icons"             => Vec::<String>::new(),
                "no_battery"        => "NO BATT",
                "notifier_critical" => 10,
                "notifier_levels"   => vec![2, 5, 10, 15, 20],
                "separator"         => " · ",
            ),
        )?;
        Ok(())
    }
}
