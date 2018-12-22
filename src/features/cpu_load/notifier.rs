use super::FEATURE_NAME;
use communication;
use error::*;
use std::sync::mpsc;
use wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    tx: mpsc::Sender<communication::Message>,
    update_interval: u64,
}

impl Notifier {
    pub(super) fn new(
        id: usize,
        tx: mpsc::Sender<communication::Message>,
        update_interval: u64,
    ) -> Self {
        Self {
            id,
            tx,
            update_interval,
        }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        loop {
            thread::sleep_secs(self.update_interval);

            communication::send_message(FEATURE_NAME, self.id, &self.tx)?;
        }
    }
}
