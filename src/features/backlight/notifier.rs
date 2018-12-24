use communication;
use error::*;
use wrapper::channel;
use wrapper::inotify;
use wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
    brightness_file: String,
}

impl Notifier {
    pub(super) fn new(
        id: usize,
        sender: channel::Sender<communication::Message>,
        brightness_file: String,
    ) -> Self {
        Self {
            id,
            sender,
            brightness_file,
        }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        let mut inotify = inotify::Inotify::init()?;

        inotify.add_watch(&self.brightness_file, inotify::WatchMask::MODIFY)?;

        inotify.listen_for_any_events(|| {
            communication::send_message(self.id, &self.sender)?;

            thread::sleep_prevent_spam();

            Ok(())
        })
    }
}
