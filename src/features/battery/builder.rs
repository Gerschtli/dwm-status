use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::path::Path;

use error::*;
use system::value_from_file;
use features::{Feature, FeatureBuilder};
use super::{Battery, BatteryInfo};

pub struct BatteryBuilder;

impl FeatureBuilder for BatteryBuilder {
    type Data = Battery;

    fn build(&self) -> Result<Self::Data> {
        if !Path::new("/sys/class/power_supply/BAT1").exists() {
            return Ok(Battery::NoBattery);
        }

        let current_now = value_from_file::<i32>("/sys/class/power_supply/BAT1/current_now").unwrap();

        if current_now == 0 {
            return Ok(Battery::Full);
        }

        let ac_online   = value_from_file::<i32>("/sys/class/power_supply/ACAD/online").map(|v| v == 1).unwrap();
        let charge_full = value_from_file::<i32>("/sys/class/power_supply/BAT1/charge_full").unwrap();
        let charge_now  = value_from_file::<i32>("/sys/class/power_supply/BAT1/charge_now").unwrap();

        let info = BatteryInfo {
            estimation: time(ac_online, charge_full, charge_now, current_now),
            percentage: capacity(charge_full, charge_now),
        };

        Ok(
            match ac_online {
                true  => Battery::Charging(info),
                false => Battery::Discharging(info),
            }
        )
    }

    fn wait_for_update(&self, tx: &Sender<Feature>) -> Result<()> {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Feature::Battery(Some(self.build().unwrap()));
            tx.send(message).unwrap();
        }
    }
}

fn capacity(charge_full: i32, charge_now: i32) -> f32 {
    charge_now as f32 / charge_full as f32
}

fn time(on_ac: bool, charge_full: i32, charge_now: i32, current_now: i32) -> Duration {
    if on_ac {
        // Charge time
        Duration::from_secs((charge_full - charge_now).abs() as u64 * 3600u64 / current_now as u64)
    }
    else {
        // Discharge time
        Duration::from_secs(charge_now as u64 * 3600u64 / current_now as u64)
    }
}
