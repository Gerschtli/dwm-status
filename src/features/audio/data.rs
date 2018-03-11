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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", AudioData::Mute), "MUTE");
        assert_eq!(format!("{}", AudioData::Volume(0)), "S 0%");
        assert_eq!(format!("{}", AudioData::Volume(85)), "S 85%");
    }
}
