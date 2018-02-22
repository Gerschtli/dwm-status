use std::fmt;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use system::value_from_file;
use super::{Feature, Message};

#[derive(Debug)]
pub struct Backlight(f32);

impl fmt::Display for Backlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L {:.0}%", self.0 * 100.0)
    }
}

impl Feature for Backlight {
    fn is_enabled() -> bool {
        Path::new("/sys/class/backlight/intel_backlight/max_brightness").exists()
    }

    fn init() -> Self {
        let max     = value_from_file::<i32>("/sys/class/backlight/intel_backlight/max_brightness").unwrap();
        let current = value_from_file::<i32>("/sys/class/backlight/intel_backlight/actual_brightness").unwrap();

        Backlight(current as f32 / max as f32)
    }

    fn wait_for_update(tx: &Sender<Message>) {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Message::Backlight(Self::init());
            tx.send(message).unwrap();
        }
    }
}
