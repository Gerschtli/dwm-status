use super::BatteryInfo;
use super::BatteryManager;
use super::BatteryNotifier;
use super::Data;
use error::*;
use feature::Updatable;
use std::collections::HashMap;

pub(super) struct Updater {
    manager: BatteryManager,
    notifier: BatteryNotifier,
}

impl Updater {
    pub(super) fn new(manager: BatteryManager, notifier: BatteryNotifier) -> Self {
        Self { manager, notifier }
    }
}

impl Updatable for Updater {
    type Data = Data;

    fn update(&mut self) -> Result<Self::Data> {
        self.manager.update_devices_list()?;

        let ac_online = self.manager.is_ac_online()?;
        let mut batteries = HashMap::new();

        for (name, device) in self.manager.get_devices() {
            let info = BatteryInfo {
                capacity: device.capacity()?,
                estimation: device.estimation(ac_online)?,
            };

            batteries.insert(name.clone(), info);
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
            }) = infos.get(0)
            {
                self.notifier.update(capacity, estimation)?;
            }
        }

        Ok(Data {
            ac_online,
            batteries,
        })
    }
}
