use super::FEATURE_NAME;
use error::*;
use io;

const PATH: &str = "/sys/class/backlight/intel_backlight";

#[derive(Debug)]
pub struct BacklightDevice {
    max: i32,
}

impl BacklightDevice {
    pub fn new() -> Result<Self> {
        Ok(BacklightDevice {
            max: get_brightness("max")?,
        })
    }

    pub fn brightness_file(&self) -> String {
        build_path("actual")
    }

    pub fn value(&self) -> Result<f32> {
        let current = get_brightness("actual")?;
        let value = current as f32 / self.max as f32;

        Ok(value)
    }
}

fn build_path(name: &str) -> String {
    format!("{}/{}_brightness", PATH, name)
}

fn get_brightness(name: &str) -> Result<i32> {
    let brightness = io::read_int_from_file(&build_path(name))
        .wrap_error(FEATURE_NAME, &format!("error reading {} brightness", name))?;

    Ok(brightness)
}
