use config::Config;
use config::ConfigError;
use error::*;
use std::result;

#[derive(Debug)]
pub struct Conf {
    config: Config,
}

impl Conf {
    pub fn new() -> Result<Self> {
        Ok(Conf {
            config: default_config().wrap_error("config", "error setting default value")?,
        })
    }
}

fn default_config() -> result::Result<Config, ConfigError> {
    let mut config = Config::default();

    config.set_default("delimiter", " / ")?;

    Ok(config)
}
