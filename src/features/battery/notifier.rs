use super::fmt_capacity;
use super::fmt_time;
use io;
use libnotify;
use settings;
use std::time;

#[derive(Debug)]
pub struct BatteryNotifier {
    capacity: Option<f32>,
    settings: settings::Battery,
}

impl BatteryNotifier {
    pub fn new(settings: settings::Battery) -> Self {
        BatteryNotifier {
            capacity: None,
            settings,
        }
    }

    pub fn reset(&mut self) {
        self.capacity = None;
    }

    pub fn update(&mut self, capacity: f32, estimation: &time::Duration) {
        if !self.settings.enable_notifier {
            return;
        }

        for level in &self.settings.notifier_levels {
            let decimal_level = *level as f32 / 100.;

            if decimal_level >= capacity {
                if match self.capacity {
                    Some(value) if decimal_level >= value => false,
                    _ => true,
                } {
                    io::show_notification(
                        &format!("Battery under {}", fmt_capacity(decimal_level)),
                        &format!("{} remaining", fmt_time(estimation)),
                        if level <= &self.settings.notifier_critical {
                            libnotify::Urgency::Critical
                        } else {
                            libnotify::Urgency::Normal
                        },
                    );
                }

                break;
            }
        }

        self.capacity = Some(capacity);
    }
}
