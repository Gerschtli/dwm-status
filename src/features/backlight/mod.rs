mod data;
mod device;
mod feature;

pub(self) use self::data::BacklightData;
pub(self) use self::device::BacklightDevice;
pub(super) use self::feature::Feature;

pub(super) const FEATURE_NAME: &str = "backlight";
