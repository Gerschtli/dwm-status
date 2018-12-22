use super::FEATURE_NAME;
use super::POWER_SUPPLY_PATH;
use error::*;
use io;
use std::time;

pub(super) fn fmt_capacity(capacity: u32) -> String {
    format!("{}%", capacity)
}

pub(super) fn fmt_time(duration: &time::Duration) -> String {
    let minutes = duration.as_secs() / 60;
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

pub(super) fn get_value(device: &str, name: &str) -> Result<u32> {
    io::read_int_from_file(&format!("{}/{}/{}", POWER_SUPPLY_PATH, device, name))
        .wrap_error(FEATURE_NAME, format!("error reading {}/{}", device, name))
}

pub(super) fn get_value2(device: &str, name1: &str, name2: &str) -> Result<u32> {
    if let Ok(result) = get_value(device, name1) {
        return Ok(result);
    }

    if let Ok(result) = get_value(device, name2) {
        return Ok(result);
    }

    Err(Error::new_custom(
        FEATURE_NAME,
        format!("error reading {}/{} or {}/{}", device, name1, device, name2),
    ))
}
