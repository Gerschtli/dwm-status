use crate::communication;
use crate::error::Result;
use crate::feature;
use crate::wrapper::channel;

pub(crate) use self::config::ConfigEntry;
use self::data::Data;
use self::notifier::Notifier;
use self::updater::Updater;

mod config;
mod data;
mod notifier;
mod updater;

pub(super) const FEATURE_NAME: &str = "time";

#[allow(clippy::unnecessary_wraps)]
pub(super) fn create(
    id: usize,
    sender: &channel::Sender<communication::Message>,
    settings: &ConfigEntry,
) -> Result<Box<dyn feature::Feature>> {
    let data = Data::new(settings.format.clone());

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        Notifier::new(id, sender.clone(), settings.update_seconds),
        Updater::new(data),
    )))
}
