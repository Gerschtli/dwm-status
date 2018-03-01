use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct BatteryInfo {
    pub estimation: Duration,
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
pub enum Battery {
    Charging(BatteryInfo),
    Discharging(BatteryInfo),
    Full,
    NoBattery,
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Battery::Charging(ref info)    => write!(f, "+ {}", info),
            Battery::Discharging(ref info) => write!(f, "- {}", info),
            Battery::Full                  => write!(f, "= 100%"),
            Battery::NoBattery             => write!(f, "NO BATT"),
        }
    }
}
