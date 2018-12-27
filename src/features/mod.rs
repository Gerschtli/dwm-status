pub(super) mod audio;
pub(super) mod backlight;
pub(super) mod battery;
pub(super) mod cpu_load;
pub(super) mod network;
pub(super) mod time;

use communication;
use error::*;
use feature;
use settings;
use wrapper::channel;

macro_rules! features {
    ( $id:expr, $name:expr, $sender:expr, $settings:expr; $( $mod:ident, )* ) => {
        match &$name.to_lowercase()[..] {
            $(
                $mod::FEATURE_NAME => $mod::create($id, $sender, &$settings.$mod),
            )*
            _ => Err(Error::new_custom(
                "create feature",
                format!("feature {} does not exist", $name),
            )),
        }
    }
}

pub(super) fn create_feature(
    id: usize,
    name: &str,
    sender: &channel::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    features!(id, name, sender, settings;
        audio,
        backlight,
        battery,
        cpu_load,
        network,
        time,
    )
}
