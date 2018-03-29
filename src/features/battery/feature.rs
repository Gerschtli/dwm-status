use super::BatteryData;
use super::BatteryInfo;
use super::BatteryManager;
use super::DbusWatcher;
use super::DeviceMessage;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub struct Battery {
    data: BatteryData,
    id: String,
    manager: BatteryManager,
    tx: mpsc::Sender<async::Message>,
    tx_devices: mpsc::Sender<DeviceMessage>,
}

impl feature::FeatureConfig for Battery {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        let (tx_devices, rx_devices) = mpsc::channel();

        let mut manager = BatteryManager::new(rx_devices)?;
        manager.update_devices_list()?;

        Ok(Battery {
            data: BatteryData {
                ac_online: true,
                batteries: HashMap::new(),
            },
            id,
            manager,
            tx,
            tx_devices,
        })
    }
}

impl feature::Feature for Battery {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id.clone();
        let tx = self.tx.clone();
        let tx_devices = self.tx_devices.clone();

        thread::spawn(move || {
            DbusWatcher::new(id, tx, tx_devices)
                .and_then(|dw| dw.start())
                .show_error()
                .unwrap();
        });

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.manager.update_devices_list()?;

        let ac_online = self.manager.is_ac_online()?;
        let mut batteries = HashMap::new();

        for (name, device) in self.manager.get_devices() {
            let info = BatteryInfo {
                capacity: device.capacity()?,
                estimation: device.estimation(ac_online)?,
            };

            if ac_online {
                device.notifier().reset();
            } else {
                device.notifier().update(info.capacity, &info.estimation);
            }

            batteries.insert((*name).to_owned(), info);
        }

        self.data.ac_online = ac_online;
        self.data.batteries = batteries;

        Ok(())
    }
}
