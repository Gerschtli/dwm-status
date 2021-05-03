use log::warn;
use uom::si::f32::Ratio;
use uom::si::f32::Time;
use uom::si::time::second;

use crate::error::Result;
use crate::error::WrapErrorExt;

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
    Unknown {
        percentage: Ratio,
    },
    Empty,
    Full,
}

pub(crate) fn all_batteries() -> Result<Vec<Battery>> {
    let manager = battery::Manager::new().wrap_error("battery", "error in loading battery info")?;

    Ok(manager
        .batteries()
        .wrap_error("battery", "error in loading battery info")?
        .filter_map(|maybe_battery| match maybe_battery {
            #[allow(clippy::match_wildcard_for_single_variants)]
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
                battery::State::Unknown => Some(Battery::Unknown {
                    // Unknown can mean either controller returned unknown,
                    // or not able to retrieve state due to some error.
                    // Nevertheless, it should be possible to get the state of
                    // charge.
                    percentage: battery.state_of_charge(),
                }),
                _ => {
                    // battery::State is non-exhaustive so we should handle this case
                    warn!("An hunandled state was reported when reading battery data");
                    None
                },
            },
            Err(err) => {
                warn!("An error occurred reading battery data: {}", err);
                None
            },
        })
        .collect::<Vec<_>>())
}
