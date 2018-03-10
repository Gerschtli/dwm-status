pub mod audio;
pub mod backlight;
pub mod battery;
pub mod time;

pub use self::audio::Audio;
pub use self::backlight::Backlight;
pub use self::battery::Battery;
pub use self::time::Time;
use async;
use error::*;
use feature;
use std::sync::mpsc;
use uuid;

macro_rules! feature {
    ($type:ident, $tx:expr) => {{
        let id = uuid::Uuid::new_v4().simple().to_string();
        Ok(
            Box::new(
                <$type as feature::FeatureConfig>::new(id, $tx.clone())?
            )
        )
    }}
}

pub fn create_feature(
    name: &str,
    tx: &mpsc::Sender<async::Message>,
) -> Result<Box<feature::Feature>> {
    match name {
        "audio" => feature!(Audio, tx),
        "backlight" => feature!(Backlight, tx),
        "battery" => feature!(Battery, tx),
        "time" => feature!(Time, tx),
        _ => Err(Error::new_custom(
            "create feature",
            &format!("feature {} does not exist", name),
        )),
    }
}
