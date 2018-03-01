use std::fmt;

#[derive(Debug)]
pub enum Audio {
    Mute,
    Volume(u32),
}

impl fmt::Display for Audio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Audio::Mute           => write!(f, "MUTE"),
            Audio::Volume(volume) => write!(f, "S {}%", volume),
        }
    }
}
