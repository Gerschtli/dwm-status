mod data;
mod device;
mod feature;

pub(self) use self::data::BacklightData;
pub(self) use self::device::BacklightDevice;
pub(crate) use self::feature::Backlight;

pub(self) const FEATURE_NAME: &str = "backlight";
