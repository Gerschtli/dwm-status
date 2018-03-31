use feature;

#[derive(Debug)]
pub enum AudioData {
    Mute,
    Volume(u32),
}

impl feature::Renderable for AudioData {
    fn render(&self) -> String {
        match *self {
            AudioData::Mute => String::from("MUTE"),
            AudioData::Volume(volume) => format!("S {}%", volume),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use feature::Renderable;
    #[test]
    fn test_display() {
        assert_eq!(AudioData::Mute.render(), "MUTE");
        assert_eq!(AudioData::Volume(0).render(), "S 0%");
        assert_eq!(AudioData::Volume(85).render(), "S 85%");
    }
}
