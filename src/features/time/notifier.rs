use chrono::Timelike;

use crate::communication;
use crate::error::Result;
use crate::wrapper::channel;
use crate::wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
    update_seconds: bool,
}

impl Notifier {
    pub(super) const fn new(
        id: usize,
        sender: channel::Sender<communication::Message>,
        update_seconds: bool,
    ) -> Self {
        Self {
            id,
            sender,
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

            communication::send_message(self.id, &self.sender)?;
        }
    }
}
