use super::BatteryNotifier;
use super::get_value;
use error::*;
use std::time;

const CHARGE_FULL: &str = "charge_full";
const CHARGE_NOW: &str = "charge_now";
const CURRENT_NOW: &str = "current_now";

#[derive(Debug)]
pub struct BatteryDevice {
    name: String,
    notifier: BatteryNotifier,
}

impl BatteryDevice {
    pub fn new(name: &str) -> Result<Self> {
        Ok(BatteryDevice {
            name: String::from(name),
            notifier: BatteryNotifier::new(),
        })
    }

    pub fn capacity(&self) -> Result<f32> {
        let charge_now = get_value(&self.name, CHARGE_NOW)?;
        let charge_full = get_value(&self.name, CHARGE_FULL)?;

        Ok(charge_now as f32 / charge_full as f32)
    }

    pub fn estimation(&self, is_ac_online: bool) -> Result<time::Duration> {
        let charge_now = get_value(&self.name, CHARGE_NOW)?;
        let current_now = get_value(&self.name, CURRENT_NOW)?;

        let seconds = if is_ac_online {
            let charge_full = get_value(&self.name, CHARGE_FULL)?;
            (charge_full - charge_now).abs() as u64 * 3600u64 / current_now as u64
        } else {
            charge_now as u64 * 3600u64 / current_now as u64
        };

        Ok(time::Duration::from_secs(seconds))
    }

    pub fn notifier(&mut self) -> &mut BatteryNotifier {
        &mut self.notifier
    }
}
