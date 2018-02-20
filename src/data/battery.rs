use std::fmt;
use std::sync::mpsc::Sender;

use super::{Feature, Message};

#[derive(Debug)]
pub struct BatteryInfo {
    estimation: u32,
    percentage: u8,
}

impl fmt::Display for BatteryInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%, ({})", self.percentage, self.estimation)
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

impl Feature for Battery {
    fn init() -> Self {
        // TODO: replace mock
        Battery::NoBattery
    }

    fn wait_for_update(tx: &Sender<Message>) {
        // TODO
    }
}
