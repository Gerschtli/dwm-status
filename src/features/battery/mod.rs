mod ac_adapter;
mod config;
mod data;
mod dbus;
mod device;
mod manager;
mod notifier;
mod updater;
mod util;

use communication;
use error::*;
use feature;
use settings;
use std::sync::mpsc;

pub(self) use self::ac_adapter::AcAdapter;
pub(crate) use self::config::ConfigEntry;
pub(self) use self::data::BatteryInfo;
pub(self) use self::data::Data;
pub(self) use self::dbus::DbusWatcher;
pub(self) use self::dbus::DeviceMessage;
pub(self) use self::device::BatteryDevice;
pub(self) use self::manager::BatteryManager;
pub(self) use self::notifier::BatteryNotifier;
pub(self) use self::updater::Updater;
pub(self) use self::util::fmt_capacity;
pub(self) use self::util::fmt_time;
pub(self) use self::util::get_value;
pub(self) use self::util::get_value2;

pub(super) const FEATURE_NAME: &str = "battery";
pub(self) const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply";

pub(super) fn create(
    id: usize,
    tx: &mpsc::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    let (tx_devices, rx_devices) = mpsc::channel();

    let manager = BatteryManager::new(settings.battery.debug, rx_devices)?;
    let notifier = BatteryNotifier::new(settings.battery.clone())?;

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        DbusWatcher::new(id, tx.clone(), tx_devices.clone())?,
        Updater::new(manager, notifier),
    )))
}
