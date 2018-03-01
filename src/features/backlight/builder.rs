use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use system::value_from_file;

use error::*;
use features::{Feature, FeatureBuilder};
use super::Backlight;

pub struct BacklightBuilder;

impl FeatureBuilder for BacklightBuilder {
    type Data = Backlight;

    fn build(&self) -> Result<Self::Data> {
        let max     = value_from_file::<i32>("/sys/class/backlight/intel_backlight/max_brightness").unwrap();
        let current = value_from_file::<i32>("/sys/class/backlight/intel_backlight/actual_brightness").unwrap();

        Ok(Backlight(current as f32 / max as f32))
    }

    fn wait_for_update(&self, tx: &Sender<Feature>) -> Result<()> {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Feature::Backlight(Some(self.build().unwrap()));
            tx.send(message).unwrap();
        }
    }
}
