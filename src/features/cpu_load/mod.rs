mod data;
mod notifier;
mod updater;

use communication;
use error::*;
use feature;
use settings;
use std::sync::mpsc;

pub(self) use self::data::Data;
pub(self) use self::notifier::Notifier;
pub(self) use self::updater::Updater;

pub(super) const FEATURE_NAME: &str = "cpu_load";

pub(super) fn create(
    id: usize,
    tx: &mpsc::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        Notifier::new(id, tx.clone(), settings.cpu_load.update_interval),
        Updater,
    )))
}
