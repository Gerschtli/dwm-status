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
use settings;
use std::sync::mpsc;
use uuid;

macro_rules! feature {
    ($name:ident, $tx:expr, $settings:expr) => {{
        let id = uuid::Uuid::new_v4().simple().to_string();
        Ok(Box::new(<$name as feature::FeatureConfig>::new(
            id,
            $tx.clone(),
            $settings.clone(),
        )?))
    }};
}

macro_rules! features {
    ($name: ident, $tx: ident, $settings: ident; $( $feature_name: expr => $struct: ident, )*) => {{
        match &$name.to_string().to_lowercase()[..] {
            $( "$feature_name" => feature!($struct, $tx, $settings.$feature_name), )*
            _ => Err(Error::new_custom(
                "create feature",
                &format!("feature {} does not exist", $name),
            )),
        }
    }}
}

pub fn create_feature(
    name: &str,
    tx: &mpsc::Sender<async::Message>,
    settings: &settings::Settings,
) -> Result<Box<feature::Feature>> {
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
