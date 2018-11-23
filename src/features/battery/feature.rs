use super::BatteryData;
use super::BatteryInfo;
use super::BatteryManager;
use super::BatteryNotifier;
use super::DbusWatcher;
use super::DeviceMessage;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use settings;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use uuid;

#[derive(Debug)]
pub struct Battery {
    data: BatteryData,
    id: uuid::Uuid,
    manager: BatteryManager,
    notifier: BatteryNotifier,
    settings: settings::Battery,
    tx_devices: mpsc::Sender<DeviceMessage>,
    tx: mpsc::Sender<async::Message>,
}

renderable_impl!(Battery);

impl feature::FeatureConfig for Battery {
    type Settings = settings::Battery;

    fn new(
        id: uuid::Uuid,
        tx: mpsc::Sender<async::Message>,
        settings: Self::Settings,
    ) -> Result<Self> {
        let (tx_devices, rx_devices) = mpsc::channel();

        let mut manager = BatteryManager::new(settings.debug, rx_devices)?;
        manager.update_devices_list()?;

        Ok(Battery {
            data: BatteryData {
                ac_online: true,
                batteries: HashMap::new(),
                settings: settings.clone(),
            },
            id,
            manager,
            notifier: BatteryNotifier::new(settings.clone()),
            settings,
            tx,
            tx_devices,
        })
    }
}

impl feature::Feature for Battery {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id;
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
                icons: self.settings.icons.clone(),
            };

            batteries.insert(String::from(&name[..]), info);
        }

        if ac_online {
            self.notifier.reset();
        } else {
            // get battery with highest capacity
            let mut infos = batteries.values().collect::<Vec<_>>();
            infos.sort_by(|a, b| b.capacity.partial_cmp(&a.capacity).unwrap());

            if let Some(&&BatteryInfo {
                capacity,
                estimation: Some(ref estimation),
                ..
            }) = infos.get(0)
            {
                self.notifier.update(capacity, estimation);
            }
        }

        self.data.ac_online = ac_online;
        self.data.batteries = batteries;

        Ok(())
    }
}
