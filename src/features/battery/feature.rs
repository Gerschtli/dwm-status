use super::BatteryData;
use super::BatteryDevice;
use super::BatteryInfo;
use super::FEATURE_NAME;
use async;
use dbus;
use error::*;
use feature;
use std::sync::mpsc;
use std::thread;
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
        let tx = self.tx.clone();
        let id = self.id.clone();
        let dbus_match = self.device.build_dbus_match();

        thread::spawn(move || {
            let connection = dbus::Connection::get_private(dbus::BusType::System)
                .wrap_error_kill(FEATURE_NAME, "failed to connect to dbus");
            connection.add_match(&dbus_match)
                .wrap_error_kill(FEATURE_NAME, "failed to add interface");

            loop {
                for item in connection.iter(100_000) {
                    if let dbus::ConnectionItem::Signal(_) = item {
                        // wait for /sys/class/power_supply files updates
                        thread::sleep(time::Duration::from_secs(1));
                        async::send_message(FEATURE_NAME, &id, &tx);
                    }
                }
            }
        });

        Ok(())
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
