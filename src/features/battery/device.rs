use super::get_value2;
use crate::error::*;
use std::cmp;
use std::time;

const CHARGE_FULL: &str = "charge_full";
const CHARGE_NOW: &str = "charge_now";
const CURRENT_NOW: &str = "current_now";
const ENERGY_FULL: &str = "energy_full";
const ENERGY_NOW: &str = "energy_now";
const POWER_NOW: &str = "power_now";

pub(super) struct BatteryDevice {
    name: String,
}

impl BatteryDevice {
    pub(super) fn new<I: Into<String>>(name: I) -> Self {
        Self { name: name.into() }
    }

    pub(super) fn capacity(&self) -> Result<u64> {
        let charge_now = get_value2(&self.name, CHARGE_NOW, ENERGY_NOW)?;
        let charge_full = get_value2(&self.name, CHARGE_FULL, ENERGY_FULL)?;

        let capacity = charge_now * 100 / charge_full;

        Ok(cmp::min(capacity, 100))
    }

    pub(super) fn estimation(&self, is_ac_online: bool) -> Result<Option<time::Duration>> {
        let current_now = get_value2(&self.name, CURRENT_NOW, POWER_NOW)?;

        if current_now == 0 {
            return Ok(None);
        }

        let charge_now = get_value2(&self.name, CHARGE_NOW, ENERGY_NOW)?;

        let seconds = if is_ac_online {
            let charge_full = get_value2(&self.name, CHARGE_FULL, ENERGY_FULL)?;
            (charge_full - charge_now) * 3600 / current_now
        } else {
            charge_now * 3600 / current_now
        };

        Ok(Some(time::Duration::from_secs(seconds)))
    }
}
