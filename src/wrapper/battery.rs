use crate::error::*;
use log::warn;
use uom::si::f32::Ratio;
use uom::si::f32::Time;
use uom::si::time::second;

#[derive(Debug)]
pub(crate) enum Battery {
    Charging {
        percentage: Ratio,
        time_to_full: Time,
    },
    Discharging {
        percentage: Ratio,
        time_to_empty: Time,
    },
    Empty,
    Full,
}

pub(crate) fn all_batteries() -> Result<Vec<Battery>> {
    let manager = battery::Manager::new().wrap_error("battery", "error in loading battery info")?;

    Ok(manager
        .batteries()
        .wrap_error("battery", "error in loading battery info")?
        .flat_map(|maybe_battery| match maybe_battery {
            Ok(battery) => match battery.state() {
                battery::State::Charging => Some(Battery::Charging {
                    percentage: battery.state_of_charge(),
                    time_to_full: battery
                        .time_to_full()
                        .unwrap_or_else(|| Time::new::<second>(0.)),
                }),
                battery::State::Discharging => Some(Battery::Discharging {
                    percentage: battery.state_of_charge(),
                    time_to_empty: battery
                        .time_to_empty()
                        .unwrap_or_else(|| Time::new::<second>(0.)),
                }),
                battery::State::Empty => Some(Battery::Empty),
                battery::State::Full => Some(Battery::Full),
                _ => None,
            },
            Err(err) => {
                warn!("An error occurred reading battery data: {}", err);
                None
            },
        })
        .collect::<Vec<_>>())
}
