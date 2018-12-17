use super::TimeData;
use super::FEATURE_NAME;
use chrono;
use chrono::Timelike;
use communication;
use error::*;
use feature;
use settings;
use std::sync::mpsc;
use std::thread;
use std::time;

#[derive(Debug)]
pub(crate) struct Time {
    id: usize,
    settings: settings::Time,
    tx: mpsc::Sender<communication::Message>,
}

impl feature::FeatureConfig for Time {
    type Settings = settings::Time;

    fn new(
        id: usize,
        tx: mpsc::Sender<communication::Message>,
        settings: Self::Settings,
    ) -> Result<Self> {
        Ok(Self { id, settings, tx })
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

            communication::send_message(FEATURE_NAME, id, &tx);
        });

        Ok(())
    }

    fn update(&mut self) -> Result<Box<dyn feature::Renderable>> {
        Ok(Box::new(TimeData(chrono::Local::now())))
    }
}
