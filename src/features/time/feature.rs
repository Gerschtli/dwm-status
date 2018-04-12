use super::TimeData;
use super::FEATURE_NAME;
use async;
use chrono;
use error::*;
use feature;
use std::sync::mpsc;
use std::thread;
use std::time;

#[derive(Debug)]
pub struct Time {
    data: TimeData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

renderable_impl!(Time);

impl feature::FeatureConfig for Time {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Time {
            data: TimeData(chrono::Local::now()),
            id,
            tx,
        })
    }
}

impl feature::Feature for Time {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let tx = self.tx.clone();
        let id = self.id.clone();

        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_secs(60));

            async::send_message(FEATURE_NAME, &id, &tx);
        });

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.data = TimeData(chrono::Local::now());
        Ok(())
    }
}
