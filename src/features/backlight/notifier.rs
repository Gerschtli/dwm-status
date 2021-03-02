use crate::communication;
use crate::error::Result;
use crate::wrapper::channel;
use crate::wrapper::inotify;
use crate::wrapper::thread;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
    brightness_file: String,
}

impl Notifier {
    pub(super) const fn new(
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
