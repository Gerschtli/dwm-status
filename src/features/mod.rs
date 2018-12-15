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
    ($name:ident, $tx:expr, $settings:expr) => {{
        Ok(Box::new(<$name as ::feature::FeatureConfig>::new(
            ::uuid::Uuid::new_v4(),
            $tx.clone(),
            $settings.clone(),
        )?))
    }};
}

pub(crate) fn create_feature(
    name: &str,
    tx: &mpsc::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    match &name.to_string().to_lowercase()[..] {
        "audio" => feature!(Audio, tx, settings.audio),
        "backlight" => feature!(Backlight, tx, settings.backlight),
        "battery" => feature!(Battery, tx, settings.battery),
        "cpu_load" => feature!(CpuLoad, tx, settings.cpu_load),
        "time" => feature!(Time, tx, settings.time),
        _ => Err(Error::new_custom(
            "create feature",
            &format!("feature {} does not exist", name),
        )),
    }
}
