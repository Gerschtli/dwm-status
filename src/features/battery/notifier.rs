use super::fmt_capacity;
use super::fmt_time;
use io;
use libnotify;
use std::time;

const LEVELS: &[f32] = &[0.02, 0.05, 0.1, 0.15, 0.2];
const CRITICAL: f32 = 0.1;

#[derive(Debug)]
pub struct BatteryNotifier {
    capacity: Option<f32>,
}

impl BatteryNotifier {
    pub fn new() -> Self {
        BatteryNotifier { capacity: None }
    }

    pub fn reset(&mut self) {
        self.capacity = None;
    }

    pub fn update(&mut self, capacity: f32, estimation: &time::Duration) {
        for level in LEVELS {
            if level >= &capacity {
                if match self.capacity {
                    Some(value) if level >= &value => false,
                    _ => true,
                } {
                    io::show_notification(
                        &format!("Battery under {}", fmt_capacity(*level)),
                        &format!("{} remaining", fmt_time(estimation)),
                        if level <= &CRITICAL {
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
