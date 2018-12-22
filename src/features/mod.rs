pub(super) mod audio;
pub(super) mod backlight;
pub(super) mod battery;
pub(super) mod cpu_load;
pub(super) mod time;

use communication;
use error::*;
use feature;
use settings;
use std::sync::mpsc;

pub(super) fn create_feature(
    id: usize,
    name: &str,
    tx: &mpsc::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    match &name.to_lowercase()[..] {
        audio::FEATURE_NAME => audio::create(id, tx, settings),
        backlight::FEATURE_NAME => backlight::create(id, tx, settings),
        battery::FEATURE_NAME => battery::create(id, tx, settings),
        cpu_load::FEATURE_NAME => cpu_load::create(id, tx, settings),
        time::FEATURE_NAME => time::create(id, tx, settings),
        _ => Err(Error::new_custom(
            "create feature",
            &format!("feature {} does not exist", name),
        )),
    }
}
