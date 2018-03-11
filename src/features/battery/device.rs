use error::*;
use io;
use std::path;
use std::time;

const DEVICE_AC: &str = "ACAD";
const DEVICE_BAT: &str = "BAT1";
const PATH: &str = "/sys/class/power_supply";

#[derive(Debug)]
pub struct BatteryDevice {
    charge_full: Option<i32>,
}

impl BatteryDevice {
    pub fn new() -> Result<Self> {
        Ok(BatteryDevice { charge_full: None })
    }

    pub fn build_dbus_match(&self) -> String {
        format!(
            "path='{}',interface='{}',member='{}'",
            format!("/org/freedesktop/UPower/devices/battery_{}", DEVICE_BAT),
            "org.freedesktop.DBus.Properties",
            "PropertiesChanged",
        )
    }

    pub fn capacity(&mut self) -> Result<f32> {
        let charge_now = get_value(DEVICE_BAT, "charge_now")?;
        let charge_full = self.charge_full()?;

        Ok(charge_now as f32 / charge_full as f32)
    }

    pub fn clear_battery_data(&mut self) {
        self.charge_full = None;
    }

    pub fn estimation(&mut self) -> Result<time::Duration> {
        let charge_now = get_value(DEVICE_BAT, "charge_now")?;
        let current_now = get_value(DEVICE_BAT, "current_now")?;

        let seconds = if self.is_ac_online()? {
            let charge_full = self.charge_full()?;
            (charge_full - charge_now).abs() as u64 * 3600u64 / current_now as u64
        } else {
            charge_now as u64 * 3600u64 / current_now as u64
        };

        Ok(time::Duration::from_secs(seconds))
    }

    pub fn has_battery(&self) -> bool {
        path::Path::new(&format!("{}/{}", PATH, DEVICE_BAT)).exists()
    }

    pub fn is_ac_online(&self) -> Result<bool> {
        Ok(get_value(DEVICE_AC, "online")? == 1)
    }

    pub fn is_full(&self) -> Result<bool> {
        Ok(get_value(DEVICE_BAT, "current_now")? == 0)
    }

    fn charge_full(&mut self) -> Result<i32> {
        Ok(match self.charge_full {
            Some(value) => value,
            None => {
                let value = get_value(DEVICE_BAT, "charge_full")?;
                self.charge_full = Some(value);
                value
            }
        })
    }
}

fn get_value(device: &str, name: &str) -> Result<i32> {
    let value = io::read_int_from_file(&format!("{}/{}/{}", PATH, device, name))
        .wrap_error("battery", &format!("error reading {}/{}", device, name))?;

    Ok(value)
}
