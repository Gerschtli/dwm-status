use crate::communication;
use crate::error::Result;
use crate::feature;
use crate::wrapper::channel;

pub(crate) use self::config::ConfigEntry;
use self::config::RenderConfig;
use self::config::UpdateConfig;
use self::data::Data;
use self::notifier::Notifier;
use self::updater::Updater;

mod config;
mod data;
mod notifier;
mod updater;

pub(super) const FEATURE_NAME: &str = "network";
const PLACEHOLDER_ESSID: &str = "{ESSID}";
const PLACEHOLDER_IPV4: &str = "{IPv4}";
const PLACEHOLDER_IPV6: &str = "{IPv6}";

#[allow(clippy::unnecessary_wraps)]
pub(super) fn create(
    id: usize,
    sender: &channel::Sender<communication::Message>,
    settings: &ConfigEntry,
) -> Result<Box<dyn feature::Feature>> {
    let data = Data::new(settings.render.clone());

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        Notifier::new(id, sender.clone()),
        Updater::new(data, settings.update.clone()),
    )))
}
