use crate::communication;
use crate::error::Result;
use crate::feature;
use crate::wrapper::channel;

pub(crate) use self::config::ConfigEntry;
pub(self) use self::config::RenderConfig;
pub(self) use self::config::UpdateConfig;
pub(self) use self::data::Data;
pub(self) use self::notifier::Notifier;
pub(self) use self::updater::Updater;

mod config;
mod data;
mod notifier;
mod updater;

pub(super) const FEATURE_NAME: &str = "network";
pub(self) const PLACEHOLDER_ESSID: &str = "{ESSID}";
pub(self) const PLACEHOLDER_IPV4: &str = "{IPv4}";
pub(self) const PLACEHOLDER_IPV6: &str = "{IPv6}";

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
