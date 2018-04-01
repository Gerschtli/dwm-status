mod ac_adapter;
mod data;
mod dbus;
mod device;
mod feature;
mod manager;
mod notifier;

pub use self::ac_adapter::AcAdapter;
pub use self::data::BatteryData;
pub use self::data::BatteryInfo;
pub use self::dbus::DbusWatcher;
pub use self::dbus::DeviceMessage;
pub use self::device::BatteryDevice;
pub use self::feature::Battery;
pub use self::manager::BatteryManager;
pub use self::notifier::BatteryNotifier;
use error::*;
use io;

pub const FEATURE_NAME: &str = "battery";
pub const POWER_SUPPLY_PATH: &str = "/sys/class/power_supply";

pub fn get_value(device: &str, name: &str) -> Result<i32> {
    io::read_int_from_file(&format!("{}/{}/{}", POWER_SUPPLY_PATH, device, name))
        .wrap_error(FEATURE_NAME, &format!("error reading {}/{}", device, name))
}

pub fn get_value2(device: &str, name1: &str, name2: &str) -> Result<i32> {
    if let Ok(result) = get_value(device, name1) {
        return Ok(result);
    }

    if let Ok(result) = get_value(device, name2) {
        return Ok(result);
    }

    Err(Error::new_custom(
        FEATURE_NAME,
        &format!("error reading {}/{} or {}/{}", device, name1, device, name2),
    ))
}
