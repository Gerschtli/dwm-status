mod config;
mod data;
mod notifier;
mod updater;

use communication;
use error::*;
use feature;
use wrapper::channel;

pub(crate) use self::config::ConfigEntry;
pub(self) use self::data::Data;
pub(self) use self::notifier::Notifier;
pub(self) use self::updater::Updater;

pub(super) const FEATURE_NAME: &str = "cpu_load";

pub(super) fn create(
    id: usize,
    sender: &channel::Sender<communication::Message>,
    settings: &ConfigEntry,
) -> Result<Box<dyn feature::Feature>> {
    let data = Data::new(settings.template.clone());

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        Notifier::new(id, sender.clone(), settings.update_interval),
        Updater::new(data),
    )))
}
