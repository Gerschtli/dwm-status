use super::BatteryData;
use super::BatteryDevice;
use super::BatteryInfo;
use async;
use error::*;
use feature;
use std::sync::mpsc;
use std::time;

#[derive(Debug)]
pub struct Battery {
    data: BatteryData,
    device: BatteryDevice,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Battery {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Battery {
            data: BatteryData::NoBattery,
            device: BatteryDevice::new()?,
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
        format!("{}", self.data)
    }

    fn update(&mut self) -> Result<()> {
        if !self.device.has_battery() {
            self.device.clear_battery_data();
            self.data = BatteryData::NoBattery;
            return Ok(());
        }

        if self.device.is_full()? {
            self.data = BatteryData::Full;
            return Ok(());
        }

        let info = BatteryInfo {
            capacity: self.device.capacity()?,
            estimation: self.device.estimation()?,
        };

        self.data = if self.device.is_ac_online()? {
            BatteryData::Charging(info)
        } else {
            BatteryData::Discharging(info)
        };

        Ok(())
    }
}
