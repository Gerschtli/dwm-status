use super::FEATURE_NAME;
use communication;
use error::*;
use std::io::Read;
use std::process;
use std::sync::mpsc;
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
        let mut monitor = process::Command::new("sh")
            .args(&["-c", "stdbuf -oL alsactl monitor"])
            .stdout(process::Stdio::piped())
            .spawn()
            .wrap_error(FEATURE_NAME, "failed to start alsactl monitor")?
            .stdout
            .wrap_error(FEATURE_NAME, "failed to pipe alsactl monitor output")?;

        let mut buffer = [0; 1024];
        loop {
            if let Ok(bytes) = monitor.read(&mut buffer) {
                // reader has reached end-of-life -> thread gets killed
                if bytes == 0 {
                    break Ok(());
                }

                communication::send_message(FEATURE_NAME, self.id, &self.tx)?;
            }

            // prevent event spamming
            thread::sleep_prevent_spam();
        }
    }
}
