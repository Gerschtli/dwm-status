use crate::communication;
use crate::error::Result;
use crate::feature;
use crate::wrapper::channel;

pub(crate) use self::config::ConfigEntry;
use self::config::NotifierConfig;
use self::config::RenderConfig;
use self::data::Data;
use self::dbus::DbusWatcher;
use self::notifier::BatteryNotifier;
use self::updater::Updater;

mod config;
mod data;
mod dbus;
mod notifier;
mod updater;

pub(super) const FEATURE_NAME: &str = "battery";

pub(super) fn create(
    id: usize,
    sender: &channel::Sender<communication::Message>,
    settings: &ConfigEntry,
) -> Result<Box<dyn feature::Feature>> {
    let data = Data::new(settings.render.clone());
    let notifier = BatteryNotifier::init(settings.notifier.clone())?;

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        DbusWatcher::new(id, sender.clone()),
        Updater::new(data, notifier),
    )))
}
