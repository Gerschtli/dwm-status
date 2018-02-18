use std::fmt;

use super::Init;

#[derive(Debug)]
pub enum Audio {
    Mute,
    Volume(u8),
}

impl fmt::Display for Audio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Audio::Mute           => write!(f, "MUTE"),
            Audio::Volume(volume) => write!(f, "S: {}%", volume),
        }
    }
}

impl Init for Audio {
    fn init() -> Self {
        // TODO: replace mock
        Audio::Mute
    }
}
