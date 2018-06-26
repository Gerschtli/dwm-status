use feature;

#[derive(Debug)]
pub enum AudioData {
    Mute { template: String },
    Volume { template: String, volume: u32 },
}

impl feature::Renderable for AudioData {
    fn render(&self) -> String {
        match *self {
            AudioData::Mute { ref template } => template.clone(),
            AudioData::Volume {
                ref template,
                ref volume,
            } => template.replace("{VOL}", &volume.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use feature::Renderable;
    #[test]
    fn test_display() {
        assert_eq!(
            AudioData::Mute {
                template: String::from("ABC")
            }.render(),
            "ABC"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("xx {VOL}%"),
                volume: 0
            }.render(),
            "xx 0%"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("xx {VOL}% {VOL}"),
                volume: 10
            }.render(),
            "xx 10% 10"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("xx {WOL}%"),
                volume: 10
            }.render(),
            "xx {WOL}%"
        );
    }
}
