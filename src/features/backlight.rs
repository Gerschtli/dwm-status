use crate::communication;
use crate::error::Result;
use crate::feature;
use crate::wrapper::channel;

pub(crate) use self::config::ConfigEntry;
pub(self) use self::config::RenderConfig;
pub(self) use self::data::Data;
pub(self) use self::device::BacklightDevice;
pub(self) use self::notifier::Notifier;
pub(self) use self::updater::Updater;

mod config;
mod data;
mod device;
mod notifier;
mod updater;

pub(super) const FEATURE_NAME: &str = "backlight";

pub(super) fn create(
    id: usize,
    sender: &channel::Sender<communication::Message>,
    settings: &ConfigEntry,
) -> Result<Box<dyn feature::Feature>> {
    let data = Data::new(settings.render.clone());
    let device = BacklightDevice::init(settings)?;

    Ok(Box::new(feature::Composer::new(
        FEATURE_NAME,
        Notifier::new(id, sender.clone(), device.brightness_file()),
        Updater::new(data, device),
    )))
}
