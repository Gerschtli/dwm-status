use config::Config;
use config::ConfigError;
use config::File;
use features::audio;
use features::backlight;
use features::battery;
use features::cpu_load;
use features::time;

pub(crate) trait ConfigType {
    fn set_default(_: &mut Config) -> Result<(), ConfigError>;

    fn set_values(_: &mut Config) -> Result<(), ConfigError> {
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct General {
    pub(crate) debug: bool,
    pub(crate) order: Vec<String>,
    pub(crate) separator: String,
}

impl ConfigType for General {
    fn set_default(config: &mut Config) -> Result<(), ConfigError> {
        config
            .set_default("debug", false)?
            .set_default("order", Vec::<String>::new())?
            .set_default("separator", " / ")?;

        Ok(())
    }
}

macro_rules! settings {
    ( $( $mod:ident ),* ) => {
        #[derive(Clone, Debug, Deserialize)]
        pub(crate) struct Settings {
            #[serde(flatten)]
            pub(crate) general: General,
            $(
                pub(crate) $mod: $mod::ConfigEntry,
            )*
        }

        impl Settings {
            pub(crate) fn new(config_path: &str) -> Result<Self, ConfigError> {
                let mut config = Config::new();

                General::set_default(&mut config)?;
                $(
                    $mod::ConfigEntry::set_default(&mut config)?;
                )*

                config.merge(File::with_name(config_path))?;

                General::set_values(&mut config)?;
                $(
                    $mod::ConfigEntry::set_values(&mut config)?;
                )*

                config.try_into()
            }
        }
    }
}

settings!(audio, backlight, battery, cpu_load, time);
