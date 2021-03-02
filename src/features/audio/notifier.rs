use crate::communication;
use crate::error::Result;
use crate::wrapper::channel;
use crate::wrapper::process;
use crate::wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
}

impl Notifier {
    pub(super) const fn new(id: usize, sender: channel::Sender<communication::Message>) -> Self {
        Self { id, sender }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let command = process::Command::new("stdbuf", &["-oL", "alsactl", "monitor"]);

        command.listen_stdout(|| communication::send_message(self.id, &self.sender))
    }
}
