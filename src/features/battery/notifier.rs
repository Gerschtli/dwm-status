use std::cmp::Ordering;
use std::collections::BinaryHeap;

use uom::si::f32::Ratio;
use uom::si::f32::Time;

use crate::error::Result;
use crate::error::ResultExt;
use crate::wrapper::battery::Battery;
use crate::wrapper::libnotify;
use crate::wrapper::uom::create_ratio_by_percentage;
use crate::wrapper::uom::get_raw_hours;
use crate::wrapper::uom::get_raw_minutes;
use crate::wrapper::uom::get_raw_percent;

use super::NotifierConfig;

struct SimpleBattery {
    percentage: Ratio,
    time_to_empty: Option<Time>,
}

impl Ord for SimpleBattery {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.percentage < other.percentage {
            Ordering::Less
        } else if self.percentage == other.percentage {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for SimpleBattery {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for SimpleBattery {}

impl PartialEq for SimpleBattery {
    fn eq(&self, other: &Self) -> bool {
        self.percentage == other.percentage
    }
}

pub(super) struct BatteryNotifier {
    libnotify: libnotify::LibNotify,
    settings: NotifierConfig,
    capacity: Option<u64>,
}

impl BatteryNotifier {
    pub(super) fn init(settings: NotifierConfig) -> Result<Self> {
        Ok(Self {
            libnotify: libnotify::LibNotify::init()?,
            settings,
            capacity: None,
        })
    }

    pub(super) fn update(&mut self, batteries: &[Battery]) {
        if !self.settings.enable_notifier {
            return;
        }

        let fullest_battery: Option<SimpleBattery> = batteries
            .iter()
            .map(|battery| match *battery {
                Battery::Charging { percentage, .. } | Battery::Unknown { percentage } => {
                    SimpleBattery {
                        percentage,
                        time_to_empty: None,
                    }
                },
                Battery::Discharging {
                    percentage,
                    time_to_empty,
                } => SimpleBattery {
                    percentage,
                    time_to_empty: Some(time_to_empty),
                },
                Battery::Empty => SimpleBattery {
                    percentage: create_ratio_by_percentage(0.),
                    time_to_empty: None,
                },
                Battery::Full => SimpleBattery {
                    percentage: create_ratio_by_percentage(100.),
                    time_to_empty: None,
                },
            })
            .collect::<BinaryHeap<SimpleBattery>>()
            .pop();

        if let Some(battery) = fullest_battery {
            self.notify(&battery);
        } else {
            self.capacity = None;
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn notify(&mut self, battery: &SimpleBattery) {
        let capacity = get_raw_percent(battery.percentage) as u64;

        for level in &self.settings.notifier_levels {
            if *level >= capacity {
                if !matches!(self.capacity, Some(value) if *level >= value) {
                    self.libnotify
                        .send_notification(
                            &format!("Battery under {}%", level),
                            battery
                                .time_to_empty
                                .map(|time| {
                                    format!(
                                        "{:02}:{:02} remaining",
                                        get_raw_hours(time),
                                        get_raw_minutes(time),
                                    )
                                })
                                .as_deref(),
                            if *level <= self.settings.notifier_critical {
                                libnotify::Urgency::Critical
                            } else {
                                libnotify::Urgency::Normal
                            },
                        )
                        .show_error_and_ignore();
                }

                break;
            }
        }

        self.capacity = Some(capacity);
    }
}
