use super::BatteryData;
use super::BatteryInfo;
use async;
use error::*;
use feature;
use io;
use std::path;
use std::sync::mpsc;
use std::time;

#[derive(Debug)]
pub struct Battery {
    data: BatteryData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Battery {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Battery {
            data: BatteryData::NoBattery,
            id,
            tx,
        })
    }
}

impl feature::Feature for Battery {
    fn id(&self) -> &str {
        &self.id
    }

    fn init_notifier(&self) -> Result<()> {
        async::schedule_update(
            "battery".to_owned(),
            self.id.to_owned(),
            time::Duration::from_secs(60),
            self.tx.clone(),
        )
    }

    fn render(&self) -> String {
        format!("{}", self.data).clone()
    }

    fn update(&mut self) -> Result<()> {
        if !path::Path::new("/sys/class/power_supply/BAT1").exists() {
            self.data = BatteryData::NoBattery;
            return Ok(());
        }

        let current_now = io::read_int_from_file("/sys/class/power_supply/BAT1/current_now")
            .wrap_error("battery", "error in reading current_now")?;

        if current_now == 0 {
            self.data = BatteryData::Full;
            return Ok(());
        }

        let ac_online = io::read_int_from_file("/sys/class/power_supply/ACAD/online")
            .map(|v| v == 1)
            .wrap_error("battery", "error in reading ac online")?;
        let charge_full = io::read_int_from_file("/sys/class/power_supply/BAT1/charge_full")
            .wrap_error("battery", "error in reading charge_full")?;
        let charge_now = io::read_int_from_file("/sys/class/power_supply/BAT1/charge_now")
            .wrap_error("battery", "error in reading charge_now")?;

        let info = BatteryInfo {
            estimation: time(ac_online, charge_full, charge_now, current_now),
            percentage: capacity(charge_full, charge_now),
        };

        self.data = if ac_online {
            BatteryData::Charging(info)
        } else {
            BatteryData::Discharging(info)
        };

        Ok(())
    }
}

fn capacity(charge_full: i32, charge_now: i32) -> f32 {
    charge_now as f32 / charge_full as f32
}

fn time(on_ac: bool, charge_full: i32, charge_now: i32, current_now: i32) -> time::Duration {
    if on_ac {
        // Charge time
        time::Duration::from_secs(
            (charge_full - charge_now).abs() as u64 * 3600u64 / current_now as u64,
        )
    } else {
        // Discharge time
        time::Duration::from_secs(charge_now as u64 * 3600u64 / current_now as u64)
    }
}
