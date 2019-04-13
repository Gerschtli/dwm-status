mod ac_adapter;
mod config;
mod data;
mod dbus;
mod device;
mod manager;
mod notifier;
mod updater;
mod util;

use crate::communication;
use crate::error::*;
use crate::feature;
use crate::wrapper::channel;

pub(self) use self::ac_adapter::AcAdapter;
pub(crate) use self::config::ConfigEntry;
pub(self) use self::config::NotifierConfig;
pub(self) use self::config::RenderConfig;
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
    sender: &channel::Sender<communication::Message>,
    settings: &ConfigEntry,
) -> Result<Box<dyn feature::Feature>> {
    let (sender_devices, receiver_devices) = channel::create();

    let data = Data::new(settings.render.clone());
    let manager = BatteryManager::init(receiver_devices)?;
    let notifier = BatteryNotifier::init(settings.notifier.clone())?;

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        DbusWatcher::new(id, sender.clone(), sender_devices),
        Updater::new(data, manager, notifier),
    )))
}
