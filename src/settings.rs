use config::Config;
use config::ConfigError;
use config::File;
use config::Value;

macro_rules! map {
    ($($k: expr => $v: expr,)*) => {{
        use std::collections::HashMap;

        let mut map: HashMap<String, Value> = HashMap::new();
        $( map.insert(String::from($k), $v.into()); )*
        map
    }}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Audio {
    pub control: String,
    pub mute: String,
    pub template: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Backlight {
    pub device: String,
    pub template: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Battery {
    pub charging: String,
    #[doc(hidden)]
    pub debug: bool,
    pub discharging: String,
    pub enable_notifier: bool,
    pub no_battery: String,
    pub notifier_critical: u32,
    pub notifier_levels: Vec<u32>,
    pub separator: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CpuLoad {
    pub template: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Time {
    pub format: String,
    pub update_seconds: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub order: Vec<String>,
    pub separator: String,

    pub audio: Audio,
    pub backlight: Backlight,
    pub battery: Battery,
    pub cpu_load: CpuLoad,
    pub time: Time,
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();

        // NOTE: if default values change, document in "examples/settings"

        config.set_default("debug", false)?;
        config.set_default("order", vec![String::from("time")])?;
        config.set_default("separator", " / ")?;

        config.set_default(
            "audio",
            map!(
                "control" => "Master",
                "mute" => "MUTE",
                "template" => "S {VOL}%",
            ),
        )?;
        config.set_default(
            "backlight",
            map!(
                "device" => "intel_backlight",
                "template" => "L {BL}%",
            ),
        )?;
        config.set_default(
            "battery",
            map!(
                "charging" => "▲",
                "discharging" => "▼",
                "enable_notifier" => true,
                "no_battery" => "NO BATT",
                "notifier_critical" => 10,
                "notifier_levels" => vec![2, 5, 10, 15, 20],
                "separator" => " · ",
            ),
        )?;
        config.set_default(
            "cpu_load",
            map!(
                "template" => "{CL1} {CL5} {CL15}",
            ),
        )?;
        config.set_default(
            "time",
            map!(
                "format" => "%Y-%m-%d %H:%M",
                "update_seconds" => false,
            ),
        )?;

        config.merge(File::with_name(config_path))?;

        // propagate debug value to battery module
        let debug = config.get_bool("debug")?;
        config.set("battery.debug", debug)?;

        config.try_into()
    }
}
