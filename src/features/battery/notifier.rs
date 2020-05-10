use super::NotifierConfig;
use crate::error::*;
use crate::wrapper::battery::Battery;
use crate::wrapper::libnotify;
use crate::wrapper::uom::get_raw_hours;
use crate::wrapper::uom::get_raw_minutes;
use crate::wrapper::uom::get_raw_percent;
use uom::si::f32::Ratio;
use uom::si::f32::Time;

struct DischargingBattery {
    percentage: Ratio,
    time_to_empty: Time,
}

pub(super) struct BatteryNotifier {
    libnotify: libnotify::LibNotify,
    settings: NotifierConfig,
}

impl BatteryNotifier {
    pub(super) fn init(settings: NotifierConfig) -> Result<Self> {
        Ok(Self {
            libnotify: libnotify::LibNotify::init()?,
            settings,
        })
    }

    pub(super) fn update(&mut self, batteries: &[Battery]) {
        if !self.settings.enable_notifier {
            return;
        }

        let mut discharging: Option<DischargingBattery> = None;

        // find fullest discharging battery
        for battery in batteries {
            if let Battery::Discharging {
                percentage,
                time_to_empty,
            } = *battery
            {
                match discharging {
                    Some(DischargingBattery {
                        percentage: discharging_percentage_value,
                        ..
                    }) if discharging_percentage_value >= percentage => {},
                    _ => {
                        discharging = Some(DischargingBattery {
                            percentage,
                            time_to_empty,
                        });
                    },
                }
            }
        }

        if let Some(battery) = discharging {
            self.notify(&battery);
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn notify(&mut self, battery: &DischargingBattery) {
        let capacity = get_raw_percent(battery.percentage) as u64;

        for level in &self.settings.notifier_levels {
            if *level >= capacity {
                self.libnotify
                    .send_notification(
                        &format!("Battery under {}%", level),
                        &format!(
                            "{:02}:{:02} remaining",
                            get_raw_hours(battery.time_to_empty),
                            get_raw_minutes(battery.time_to_empty),
                        ),
                        if *level <= self.settings.notifier_critical {
                            libnotify::Urgency::Critical
                        } else {
                            libnotify::Urgency::Normal
                        },
                    )
                    .show_error_and_ignore();

                break;
            }
        }
    }
}
