use communication;
use error::*;
use wrapper::channel;
use wrapper::process;
use wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
}

impl Notifier {
    pub(super) fn new(id: usize, sender: channel::Sender<communication::Message>) -> Self {
        Self { id, sender }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let command = process::Command::new("stdbuf", &["-oL", "alsactl", "monitor"]);

        command.listen_stdout(
            || communication::send_message(self.id, &self.sender),
            thread::sleep_prevent_spam,
        )?;

        Ok(())
    }
}
