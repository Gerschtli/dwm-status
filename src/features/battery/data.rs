use std::fmt;
use std::time;

#[derive(Debug)]
pub struct BatteryInfo {
    pub estimation: time::Duration,
    pub percentage: f32,
}

impl fmt::Display for BatteryInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:.0}% ({:02}:{:02})",
            self.percentage * 100.0,
            self.estimation.as_secs() / 3600,
            self.estimation.as_secs() % 60
        )
    }
}

#[derive(Debug)]
pub enum BatteryData {
    Charging(BatteryInfo),
    Discharging(BatteryInfo),
    Full,
    NoBattery,
}

impl fmt::Display for BatteryData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatteryData::Charging(ref info) => write!(f, "+ {}", info),
            BatteryData::Discharging(ref info) => write!(f, "- {}", info),
            BatteryData::Full => write!(f, "= 100%"),
            BatteryData::NoBattery => write!(f, "NO BATT"),
        }
    }
}
