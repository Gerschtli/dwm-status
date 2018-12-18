mod audio;
mod backlight;
mod battery;
mod cpu_load;
mod time;

use communication;
use error::*;
use feature;
use settings;
use std::sync::mpsc;

macro_rules! features {
    ( $id:expr, $name:expr, $tx:expr, $settings:expr; $( $feature:ident, )* ) => {{
        match &$name.to_lowercase()[..] {
            $(
                $feature::FEATURE_NAME => {
                    Ok(
                        Box::new(
                            <$feature::Feature as feature::FeatureConfig>::new(
                                $id,
                                $tx.clone(),
                                $settings.$feature.clone(),
                            )?
                        )
                    )
                }
            )*,
            _ => Err(Error::new_custom(
                "create feature",
                &format!("feature {} does not exist", $name),
            )),
        }
    }};
}

pub(super) fn create_feature(
    id: usize,
    name: &str,
    tx: &mpsc::Sender<communication::Message>,
    settings: &settings::Settings,
) -> Result<Box<dyn feature::Feature>> {
    features!(id, name, tx, settings;
        audio,
        backlight,
        battery,
        cpu_load,
        time,
    )
}
