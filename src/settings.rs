use config::Config;
use config::ConfigError;
use config::File;

macro_rules! map {
    ($($k: expr => $v: expr),*) => {{
        use std::collections::HashMap;

        let mut map: HashMap<String, String> = HashMap::new();
        $( map.insert($k, $v); )*
        map
    }}
}

#[derive(Clone, Debug, Deserialize)]
pub struct Audio {
    pub mute: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Backlight {}

#[derive(Clone, Debug, Deserialize)]
pub struct Battery {}

#[derive(Clone, Debug, Deserialize)]
pub struct CpuLoad {}

#[derive(Clone, Debug, Deserialize)]
pub struct Time {}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub delimiter: String,
    pub order: Vec<String>,

    pub audio: Audio,
    pub backlight: Backlight,
    pub battery: Battery,
    pub cpu_load: CpuLoad,
    pub time: Time,
}

impl Settings {
    pub fn new(config_path: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();

        config.set_default("debug", false)?;
        config.set_default("delimiter", " / ")?;
        config.set_default("order", vec![String::from("time")])?;

        config.set_default("audio", map!(String::from("mute") => String::from("MUTE")))?;
        config.set_default("backlight", map!())?;
        config.set_default("battery", map!())?;
        config.set_default("cpu_load", map!())?;
        config.set_default("time", map!())?;

        config.merge(File::with_name(config_path))?;

        config.try_into()
    }
}
