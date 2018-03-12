mod data;
mod device;
mod feature;

pub use self::data::BacklightData;
pub use self::device::BacklightDevice;
pub use self::feature::Backlight;

pub const FEATURE_NAME: &str = "backlight";
