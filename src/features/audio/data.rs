use std::fmt;

#[derive(Debug)]
pub enum AudioData {
    Mute,
    Volume(u32),
}

impl fmt::Display for AudioData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AudioData::Mute => write!(f, "MUTE"),
            AudioData::Volume(volume) => write!(f, "S {}%", volume),
        }
    }
}
