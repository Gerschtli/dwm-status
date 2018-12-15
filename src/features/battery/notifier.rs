use super::fmt_capacity;
use super::fmt_time;
use error::*;
use settings;
use std::time;
use wrapper::libnotify;

#[derive(Debug)]
pub(super) struct BatteryNotifier {
    capacity: Option<f32>,
    libnotify: libnotify::LibNotify,
    settings: settings::Battery,
}

impl BatteryNotifier {
    pub(super) fn new(settings: settings::Battery) -> Result<Self> {
        Ok(BatteryNotifier {
            capacity: None,
            libnotify: libnotify::LibNotify::new()?,
            settings,
        })
    }

    pub(super) fn reset(&mut self) {
        self.capacity = None;
    }

    pub(super) fn update(&mut self, capacity: f32, estimation: &time::Duration) -> Result<()> {
        if !self.settings.enable_notifier {
            return Ok(());
        }

        for level in &self.settings.notifier_levels {
            let decimal_level = *level as f32 / 100.;

            if decimal_level >= capacity {
                if match self.capacity {
                    Some(value) if decimal_level >= value => false,
                    _ => true,
                } {
                    self.libnotify.send_notification(
                        &format!("Battery under {}", fmt_capacity(decimal_level)),
                        &format!("{} remaining", fmt_time(estimation)),
                        if *level <= self.settings.notifier_critical {
                            libnotify::Urgency::Critical
                        } else {
                            libnotify::Urgency::Normal
                        },
                    )?;
                }

                break;
            }
        }

        self.capacity = Some(capacity);

        Ok(())
    }
}
