use super::FEATURE_NAME;
use communication;
use error::*;
use std::sync::mpsc;
use wrapper::inotify;
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
        let mut inotify = inotify::Inotify::init()?;

        inotify.add_watch(&self.brightness_file, inotify::WatchMask::MODIFY)?;

        inotify.listen_for_any_events(|| {
            communication::send_message(FEATURE_NAME, self.id, &self.tx)?;

            thread::sleep_prevent_spam();

            Ok(())
        })
    }
}
