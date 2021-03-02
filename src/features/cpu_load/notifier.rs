use crate::communication;
use crate::error::Result;
use crate::wrapper::channel;
use crate::wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
    update_interval: u64,
}

impl Notifier {
    pub(super) const fn new(
        id: usize,
        sender: channel::Sender<communication::Message>,
        update_interval: u64,
    ) -> Self {
        Self {
            id,
            sender,
            update_interval,
        }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        loop {
            thread::sleep_secs(self.update_interval);

            communication::send_message(self.id, &self.sender)?;
        }
    }
}
