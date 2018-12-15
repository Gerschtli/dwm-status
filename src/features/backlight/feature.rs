use super::BacklightData;
use super::BacklightDevice;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use inotify;
use settings;
use std::sync::mpsc;
use std::thread;
use std::time;
use uuid;

#[derive(Debug)]
pub(crate) struct Backlight {
    device: BacklightDevice,
    id: uuid::Uuid,
    settings: settings::Backlight,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Backlight {
    type Settings = settings::Backlight;

    fn new(
        id: uuid::Uuid,
        tx: mpsc::Sender<async::Message>,
        settings: Self::Settings,
    ) -> Result<Self> {
        Ok(Backlight {
            device: BacklightDevice::new(&settings.device)?,
            id,
            settings,
            tx,
        })
    }
}

impl feature::Feature for Backlight {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id;
        let tx = self.tx.clone();
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
                    async::send_message(FEATURE_NAME, id, &tx);
                }

                // prevent event spamming
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        Ok(())
    }

    fn update(&mut self) -> Result<Box<dyn feature::Renderable>> {
        Ok(Box::new(BacklightData(self.device.value()?)))
    }
}
