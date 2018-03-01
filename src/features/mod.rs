pub mod audio;
pub mod backlight;
pub mod battery;
pub mod time;

use error::*;
use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum Feature {
    Audio(Option<audio::Audio>),
    Backlight(Option<backlight::Backlight>),
    Battery(Option<battery::Battery>),
    Time(Option<time::Time>),
}

pub trait FeatureBuilder {
    type Data;

    fn build(&self) -> Result<Self::Data>;

    fn wait_for_update(&self, tx: &Sender<Feature>) -> Result<()>;
}
