use super::FEATURE_NAME;
use communication;
use error::*;
use inotify;
use std::sync::mpsc;
use wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    tx: mpsc::Sender<communication::Message>,
    brightness_file: String,
}

impl Notifier {
    pub(super) fn new(
        id: usize,
        tx: mpsc::Sender<communication::Message>,
        brightness_file: String,
    ) -> Self {
        Self {
            id,
            tx,
            brightness_file,
        }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/backlight.rs
        let mut notify =
            inotify::Inotify::init().wrap_error(FEATURE_NAME, "failed to start inotify")?;
        notify
            .add_watch(&self.brightness_file, inotify::WatchMask::MODIFY)
            .wrap_error(FEATURE_NAME, "failed to watch brightness file")?;

        let mut buffer = [0; 1024];
        loop {
            let mut events = notify
                .read_events_blocking(&mut buffer)
                .wrap_error(FEATURE_NAME, "error while reading inotify events")?;

            if events.any(|event| event.mask.contains(inotify::EventMask::MODIFY)) {
                communication::send_message(FEATURE_NAME, self.id, &self.tx);
            }

            // prevent event spamming
            thread::sleep_prevent_spam();
        }
    }
}
