use super::fmt_capacity;
use super::fmt_time;
use super::ConfigEntry;
use error::*;
use std::time;
use wrapper::libnotify;

#[derive(Debug)]
pub(super) struct BatteryNotifier {
    capacity: Option<u32>,
    libnotify: libnotify::LibNotify,
    settings: ConfigEntry,
}

impl BatteryNotifier {
    pub(super) fn new(settings: ConfigEntry) -> Result<Self> {
        Ok(Self {
            capacity: None,
            libnotify: libnotify::LibNotify::new()?,
            settings,
        })
    }

    pub(super) fn reset(&mut self) {
        self.capacity = None;
    }

    pub(super) fn update(&mut self, capacity: u32, estimation: &time::Duration) -> Result<()> {
        if !self.settings.enable_notifier {
            return Ok(());
        }

        for level in &self.settings.notifier_levels {
            if *level >= capacity {
                if match self.capacity {
                    Some(value) if *level >= value => false,
                    _ => true,
                } {
                    self.libnotify.send_notification(
                        &format!("Battery under {}", fmt_capacity(*level)),
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
