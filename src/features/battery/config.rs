use super::FEATURE_NAME;
use config::Config;
use config::ConfigError;
use config::Value;
use settings::ConfigType;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ConfigEntry {
    pub(crate) charging: String,
    #[serde(skip)]
    pub(crate) debug: bool,
    pub(crate) discharging: String,
    pub(crate) enable_notifier: bool,
    pub(crate) icons: Vec<String>,
    pub(crate) no_battery: String,
    pub(crate) notifier_critical: u32,
    pub(crate) notifier_levels: Vec<u32>,
    pub(crate) separator: String,
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

    fn set_values(config: &mut Config) -> Result<(), ConfigError> {
        // propagate debug value to battery module
        let debug = config.get_bool("debug")?;
        config.set(&format!("{}.debug", FEATURE_NAME), debug)?;
        Ok(())
    }
}
