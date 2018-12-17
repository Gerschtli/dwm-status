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
use communication;
use error::*;
use feature;
use settings;
use std::sync::mpsc;

macro_rules! feature {
    ($name:ident, $id:expr, $tx:expr, $settings:expr) => {{
        Ok(Box::new(<$name as ::feature::FeatureConfig>::new(
            $id,
            $tx.clone(),
            $settings.clone(),
        )?))
    }};
}

pub(crate) fn create_feature(
    id: usize,
    name: &str,
    tx: &mpsc::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    match &name.to_lowercase()[..] {
        "audio" => feature!(Audio, id, tx, settings.audio),
        "backlight" => feature!(Backlight, id, tx, settings.backlight),
        "battery" => feature!(Battery, id, tx, settings.battery),
        "cpu_load" => feature!(CpuLoad, id, tx, settings.cpu_load),
        "time" => feature!(Time, id, tx, settings.time),
        _ => Err(Error::new_custom(
            "create feature",
            &format!("feature {} does not exist", name),
        )),
    }
}
