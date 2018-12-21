use super::FEATURE_NAME;
use chrono;
use chrono::Timelike;
use communication;
use error::*;
use std::sync::mpsc;
use wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    tx: mpsc::Sender<communication::Message>,
    update_seconds: bool,
}

impl Notifier {
    pub(super) fn new(
        id: usize,
        tx: mpsc::Sender<communication::Message>,
        update_seconds: bool,
    ) -> Self {
        Self {
            id,
            tx,
            update_seconds,
        }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        loop {
            let update_interval = if self.update_seconds {
                1
            } else {
                60 - u64::from(chrono::Local::now().second())
            };

            thread::sleep_secs(update_interval);

            communication::send_message(FEATURE_NAME, self.id, &self.tx);
        }
    }
}
