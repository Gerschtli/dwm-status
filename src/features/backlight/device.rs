use super::FEATURE_NAME;
use error::*;
use io;

#[derive(Debug)]
pub struct BacklightDevice {
    max: i32,
    path: String,
}

impl BacklightDevice {
    pub fn new(device: &str) -> Result<Self> {
        let mut device = BacklightDevice {
            max: 0,
            path: format!("/sys/class/backlight/{}", device),
        };

        device.max = device.get_brightness("max")?;

        Ok(device)
    }

    pub fn brightness_file(&self) -> String {
        self.build_path("actual")
    }

    pub fn value(&self) -> Result<f32> {
        let current = self.get_brightness("actual")?;
        let value = current as f32 / self.max as f32;

        Ok(value)
    }

    fn build_path(&self, name: &str) -> String {
        format!("{}/{}_brightness", self.path, name)
    }

    fn get_brightness(&self, name: &str) -> Result<i32> {
        let brightness = io::read_int_from_file(&self.build_path(name))
            .wrap_error(FEATURE_NAME, &format!("error reading {} brightness", name))?;

        Ok(brightness)
    }
}
