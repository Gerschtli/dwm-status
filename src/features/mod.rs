mod audio;
mod backlight;
mod battery;
mod cpu_load;
mod time;

use self::audio::Audio;
use self::backlight::Backlight;
use self::battery::Battery;
use self::cpu_load::CpuLoad;
use self::time::Time;
use async;
use error::*;
use feature;
use std::sync::mpsc;
use uuid;

macro_rules! feature {
    ($name:ident, $tx:expr) => {{
        let id = uuid::Uuid::new_v4().simple().to_string();
        Ok(Box::new(<$name as feature::FeatureConfig>::new(
            id,
            $tx.clone(),
        )?))
    }};
}

pub fn create_feature(
    name: &str,
    tx: &mpsc::Sender<async::Message>,
) -> Result<Box<feature::Feature>> {
    match &name.to_string().to_lowercase()[..] {
        "audio" => feature!(Audio, tx),
        "backlight" => feature!(Backlight, tx),
        "battery" => feature!(Battery, tx),
        "cpu_load" => feature!(CpuLoad, tx),
        "time" => feature!(Time, tx),
        _ => Err(Error::new_custom(
            "create feature",
            &format!("feature {} does not exist", name),
        )),
    }
}
