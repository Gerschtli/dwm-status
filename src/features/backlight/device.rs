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

    pub fn value(&self) -> Result<f32> {
        let current = get_brightness("actual")?;
        let value = current as f32 / self.max as f32;

        Ok(value)
    }
}

fn get_brightness(name: &str) -> Result<i32> {
    let brightness = io::read_int_from_file(&format!("{}/{}_brightness", PATH, name))
        .wrap_error("backlight", &format!("error reading {} brightness", name))?;

    Ok(brightness)
}
