mod data;
mod device;
mod feature;
mod notifier;

pub use self::data::BatteryData;
pub use self::data::BatteryInfo;
pub use self::device::BatteryDevice;
pub use self::feature::Battery;
pub use self::notifier::BatteryNotifier;

pub const FEATURE_NAME: &str = "battery";
