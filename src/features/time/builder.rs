use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use chrono::Local;

use error::*;
use features::{Feature, FeatureBuilder};
use super::Time;

pub struct TimeBuilder;

impl FeatureBuilder for TimeBuilder {
    type Data = Time;

    fn build(&self) -> Result<Self::Data> {
        Ok(Time(Local::now()))
    }

    fn wait_for_update(&self, tx: &Sender<Feature>) -> Result<()> {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Feature::Time(Some(self.build().unwrap()));
            tx.send(message).unwrap();
        }
    }
}
