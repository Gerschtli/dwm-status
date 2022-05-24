use std::path::Path;

use crate::error::Result;
use crate::error::WrapErrorExt;
use crate::features::backlight::ConfigEntry;
use crate::wrapper::file;
use glob::glob;

use super::FEATURE_NAME;

const BACKLIGHT_SYS_PATH: &str = "/sys/class/backlight";

pub(super) struct BacklightDevice {
    max: u32,
    path: String,
}

impl BacklightDevice {
    pub(super) fn init(settings: &ConfigEntry) -> Result<Self> {
        let mut device = Self {
            max: 0,
            path: Self::backlight_dir(settings)?,
        };

        device.max = device.get_brightness("max")?;

        Ok(device)
    }

    pub(super) fn backlight_dir(settings: &ConfigEntry) -> Result<String> {
        let default_path = format!("{}/{}", BACKLIGHT_SYS_PATH, settings.device);

        if Path::new(&default_path).exists() || settings.fallback.is_none() {
            return Ok(default_path);
        }

        let pattern = format!(
            "{}/{}",
            BACKLIGHT_SYS_PATH,
            settings.fallback.as_ref().unwrap()
        );

        if let Some(Ok(path)) = glob(&pattern)
            .wrap_error(FEATURE_NAME, "Failed to read glob pattern")?
            .next()
        {
            return Ok(path.display().to_string());
        }

        Ok(default_path)
    }

    pub(super) fn brightness_file(&self) -> String {
        self.build_path("actual")
    }

    pub(super) fn value(&self) -> Result<u32> {
        let current = self.get_brightness("actual")?;
        let value = current * 100 / self.max;

        Ok(value)
    }

    fn build_path(&self, name: &str) -> String {
        format!("{}/{}_brightness", self.path, name)
    }

    fn get_brightness(&self, name: &str) -> Result<u32> {
        let brightness = file::parse_file_content(self.build_path(name))
            .wrap_error(FEATURE_NAME, format!("error reading {} brightness", name))?;

        Ok(brightness)
    }
}
