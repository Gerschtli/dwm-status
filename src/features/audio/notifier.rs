use crate::communication;
use crate::error::Result;
use crate::features::audio::Backend;
use crate::wrapper::channel;
use crate::wrapper::process;
use crate::wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
    backend: Backend,
}

impl Notifier {
    pub(super) const fn new(
        id: usize,
        sender: channel::Sender<communication::Message>,
        backend: Backend,
    ) -> Self {
        Self {
            id,
            sender,
            backend,
        }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let stdbuf_options = "-oL";
        let args = match self.backend {
            Backend::Alsa => [stdbuf_options, "alsactl", "monitor"],
            Backend::Pulseaudio => [stdbuf_options, "pactl", "subscribe"],
        };

        let command = process::Command::new("stdbuf", &args);

        command.listen_stdout(|| communication::send_message(self.id, &self.sender))
    }
}
