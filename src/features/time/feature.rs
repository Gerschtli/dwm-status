use super::TimeData;
use super::FEATURE_NAME;
use async;
use chrono;
use chrono::Timelike;
use error::*;
use feature;
use settings;
use std::sync::mpsc;
use std::thread;
use std::time;
use uuid;

#[derive(Debug)]
pub(crate) struct Time {
    id: uuid::Uuid,
    settings: settings::Time,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Time {
    type Settings = settings::Time;

    fn new(
        id: uuid::Uuid,
        tx: mpsc::Sender<async::Message>,
        settings: Self::Settings,
    ) -> Result<Self> {
        Ok(Time { id, settings, tx })
    }
}

impl feature::Feature for Time {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id;
        let tx = self.tx.clone();
        let update_seconds = self.settings.update_seconds;

        thread::spawn(move || loop {
            let update_interval = if update_seconds {
                1
            } else {
                60 - u64::from(chrono::Local::now().second())
            };

            thread::sleep(time::Duration::from_secs(update_interval));

            async::send_message(FEATURE_NAME, id, &tx);
        });

        Ok(())
    }

    fn update(&mut self) -> Result<Box<dyn feature::Renderable>> {
        Ok(Box::new(TimeData(chrono::Local::now())))
    }
}
