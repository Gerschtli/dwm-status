use std::fmt;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use system::value_from_file;
use super::{Feature, Message};

#[derive(Debug)]
pub struct BatteryInfo {
    estimation: Duration,
    percentage: f32,
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

impl Feature for Battery {
    fn is_enabled() -> bool {
        Path::new("/sys/class/power_supply/ACAD").exists()
    }

    fn init() -> Self {
        if !Path::new("/sys/class/power_supply/BAT1").exists() {
            return Battery::NoBattery
        }

        let current_now = value_from_file::<i32>("/sys/class/power_supply/BAT1/current_now").unwrap();

        if current_now == 0 {
            return Battery::Full;
        }

        let ac_online   = value_from_file::<i32>("/sys/class/power_supply/ACAD/online").map(|v| v == 1).unwrap();
        let charge_full = value_from_file::<i32>("/sys/class/power_supply/BAT1/charge_full").unwrap();
        let charge_now  = value_from_file::<i32>("/sys/class/power_supply/BAT1/charge_now").unwrap();

        let info = BatteryInfo {
            estimation: time(ac_online, charge_full, charge_now, current_now),
            percentage: capacity(charge_full, charge_now),
        };

        match ac_online {
            true  => Battery::Charging(info),
            false => Battery::Discharging(info),
        }
    }

    fn wait_for_update(tx: &Sender<Message>) {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Message::Battery(Self::init());
            tx.send(message).unwrap();
        }
    }
}

fn capacity(charge_full: i32, charge_now: i32) -> f32 {
    charge_now as f32 / charge_full as f32
}

fn time(on_ac: bool, charge_full: i32, charge_now: i32, current_now: i32) -> Duration {
    if on_ac {
        // Charge time
        Duration::from_secs((charge_full - charge_now).abs() as u64 * 3600u64 / current_now as u64)
    }
    else {
        // Discharge time
        Duration::from_secs(charge_now as u64 * 3600u64 / current_now as u64)
    }
}
