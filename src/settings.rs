use config::Config;
use config::ConfigError;
use config::File;
use config::Value;

macro_rules! map {
    ( $( $k: expr => $v: expr, )* ) => {{
        use std::collections::HashMap;

        let mut map: HashMap<String, Value> = HashMap::new();
        $( map.insert(String::from($k), $v.into()); )*
        map
    }}
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Audio {
    pub(crate) control: String,
    pub(crate) mute: String,
    pub(crate) template: String,
    pub(crate) icons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Backlight {
    pub(crate) device: String,
    pub(crate) template: String,
    pub(crate) icons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Battery {
    pub(crate) charging: String,
    #[doc(hidden)]
    pub(crate) debug: bool,
    pub(crate) discharging: String,
    pub(crate) enable_notifier: bool,
    pub(crate) no_battery: String,
    pub(crate) notifier_critical: u16,
    pub(crate) notifier_levels: Vec<u16>,
    pub(crate) separator: String,
    pub(crate) icons: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CpuLoad {
    pub(crate) template: String,
    pub(crate) update_interval: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Time {
    pub(crate) format: String,
    pub(crate) update_seconds: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) debug: bool,
    pub(crate) order: Vec<String>,
    pub(crate) separator: String,

    pub(crate) audio: Audio,
    pub(crate) backlight: Backlight,
    pub(crate) battery: Battery,
    pub(crate) cpu_load: CpuLoad,
    pub(crate) time: Time,
}

impl Settings {
    pub(crate) fn new(config_path: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();

        // NOTE: if default values change, document in "examples/default-settings"

        config.set_default("debug", false)?;
        config.set_default("order", Vec::<String>::new())?;
        config.set_default("separator", " / ")?;

        config.set_default(
            "audio",
            map!(
                "control" => "Master",
                "mute" => "MUTE",
                "template" => "S {VOL}%",
                "icons" => Vec::<String>::new(),
            ),
        )?;
        config.set_default(
            "backlight",
            map!(
                "device" => "intel_backlight",
                "template" => "L {BL}%",
                "icons" => Vec::<String>::new(),
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
                "icons" => Vec::<String>::new(),
            ),
        )?;
        config.set_default(
            "cpu_load",
            map!(
                "template" => "{CL1} {CL5} {CL15}",
                "update_interval" => 20,
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

        // dynamically set time.update_seconds
        let time_format = config.get_str("time.format")?.replace("%%", "");
        if ["%f", "%r", "%S", "%s", "%T"]
            .iter()
            .any(|specifier| time_format.contains(specifier))
        {
            config.set_default("time.update_seconds", true)?;
        }

        config.try_into()
    }
}
