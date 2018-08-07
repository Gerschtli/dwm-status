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

#[derive(Debug)]
pub struct Time {
    data: TimeData,
    id: String,
    settings: settings::Time,
    tx: mpsc::Sender<async::Message>,
}

renderable_impl!(Time);

impl feature::FeatureConfig for Time {
    type Settings = settings::Time;

    fn new(id: String, tx: mpsc::Sender<async::Message>, settings: Self::Settings) -> Result<Self> {
        Ok(Time {
            data: TimeData {
                format: settings.format.clone(),
                time: chrono::Local::now(),
            },
            id,
            settings,
            tx,
        })
    }
}

impl feature::Feature for Time {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id.clone();
        let tx = self.tx.clone();
        let update_seconds = self.settings.update_seconds;

        thread::spawn(move || loop {
            async::send_message(FEATURE_NAME, &id, &tx);

            let update_interval = if update_seconds {
                1
            } else {
                60 - chrono::Local::now().second()
            };

            thread::sleep(time::Duration::from_secs(update_interval as u64));
        });

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.data = TimeData {
            format: self.settings.format.clone(),
            time: chrono::Local::now(),
        };

        Ok(())
    }
}
