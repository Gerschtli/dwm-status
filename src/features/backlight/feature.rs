use super::BacklightData;
use super::BacklightDevice;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use inotify;
use std::sync::mpsc;
use std::thread;
use std::time;

#[derive(Debug)]
pub struct Backlight {
    data: BacklightData,
    device: BacklightDevice,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Backlight {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Backlight {
            data: BacklightData(0.),
            device: BacklightDevice::new()?,
            id,
            tx,
        })
    }
}

impl feature::Feature for Backlight {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let tx = self.tx.clone();
        let id = self.id.clone();
        let brightness_file = self.device.brightness_file();

        thread::spawn(move || {
            // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/backlight.rs
            let mut notify =
                inotify::Inotify::init().wrap_error_kill(FEATURE_NAME, "failed to start inotify");
            notify
                .add_watch(brightness_file, inotify::WatchMask::MODIFY)
                .wrap_error_kill(FEATURE_NAME, "failed to watch brightness file");

            let mut buffer = [0; 1024];
            loop {
                let mut events = notify
                    .read_events_blocking(&mut buffer)
                    .wrap_error_kill(FEATURE_NAME, "error while reading inotify events");

                if events.any(|event| event.mask.contains(inotify::EventMask::MODIFY)) {
                    async::send_message(FEATURE_NAME, &id, &tx);
                }

                // prevent event spamming
                thread::sleep(time::Duration::from_millis(250));
            }
        });

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.data = BacklightData(self.device.value()?);

        Ok(())
    }
}
