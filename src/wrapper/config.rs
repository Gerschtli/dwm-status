use config;
use error::*;
use serde::Deserialize;

pub(crate) use config::Value;

const ERROR_NAME: &str = "config";

pub(crate) struct Config {
    config: config::Config,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            config: config::Config::new(),
        }
    }

    pub(crate) fn set_default<T>(&mut self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value>,
    {
        self.config
            .set_default(key, value)
            .wrap_error(ERROR_NAME, "set default failed")?;

        Ok(())
    }

    pub(crate) fn set_path(&mut self, path: &str) -> Result<()> {
        self.config
            .merge(config::File::with_name(path))
            .wrap_error(ERROR_NAME, "merge config file failed")?;

        Ok(())
    }

    pub(crate) fn get_option<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<Option<T>> {
        self.config
            .get(key)
            .wrap_error(ERROR_NAME, "read optional field failed")
    }

    pub(crate) fn get_str(&self, key: &str) -> Result<String> {
        self.config
            .get_str(key)
            .wrap_error(ERROR_NAME, "read string failed")
    }

    #[allow(single_use_lifetimes)] // FIXME
    pub(crate) fn try_into<'de, T: Deserialize<'de>>(self) -> Result<T> {
        self.config
            .try_into()
            .wrap_error(ERROR_NAME, "failed to build settings object")
    }
}
