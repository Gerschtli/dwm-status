use super::FEATURE_NAME;
use communication;
use error::*;
use std::sync::mpsc;
use wrapper::process;
use wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    tx: mpsc::Sender<communication::Message>,
}

impl Notifier {
    pub(super) fn new(id: usize, tx: mpsc::Sender<communication::Message>) -> Self {
        Self { id, tx }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let command = process::Command::new("stdbuf", &["-oL", "alsactl", "monitor"]);

        command.listen_stdout(
            || communication::send_message(FEATURE_NAME, self.id, &self.tx),
            thread::sleep_prevent_spam,
        )?;

        Ok(())
    }
}
