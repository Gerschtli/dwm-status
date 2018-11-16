use feature;
use utils::icon_by_percentage;

#[derive(Debug)]
pub enum AudioData {
    Mute {
        template: String,
    },
    Volume {
        template: String,
        volume: u32,
        icons: Vec<String>,
    },
}

impl feature::Renderable for AudioData {
    fn render(&self) -> String {
        match *self {
            AudioData::Mute { ref template } => template.clone(),
            AudioData::Volume {
                ref template,
                volume,
                ref icons,
            } => {
                let mut rendered = template.replace("{VOL}", &volume.to_string());
                let icon_optional = icon_by_percentage(&icons, volume);
                if let Some(icon) = icon_optional {
                    rendered = rendered.replace("{ICO}", icon);
                }
                rendered
            },
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
                template: String::from("ABC"),
            }
            .render(),
            "ABC"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("xx {VOL}%"),
                volume: 0,
                icons: Vec::<String>::new(),
            }
            .render(),
            "xx 0%"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("xx {VOL}% {VOL}"),
                volume: 10,
                icons: Vec::<String>::new(),
            }
            .render(),
            "xx 10% 10"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("xx {WOL}%"),
                volume: 10,
                icons: Vec::<String>::new(),
            }
            .render(),
            "xx {WOL}%"
        );
    }

    #[test]
    fn test_display_with_dynamic_icons() {
        assert_eq!(
            AudioData::Volume {
                template: String::from("{ICO} {VOL}%"),
                volume: 0,
                icons: Vec::<String>::new()
            }
            .render(),
            "{ICO} 0%"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("{ICO} {VOL}%"),
                volume: 0,
                icons: vec![
                    String::from("LOW"),
                    String::from("MIDDLE"),
                    String::from("HIGH")
                ]
            }
            .render(),
            "LOW 0%"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("{ICO} {VOL}%"),
                volume: 65,
                icons: vec![
                    String::from("LOW"),
                    String::from("MIDDLE"),
                    String::from("HIGH")
                ]
            }
            .render(),
            "MIDDLE 65%"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("{ICO} {VOL}%"),
                volume: 100,
                icons: vec![
                    String::from("LOW"),
                    String::from("MIDDLE"),
                    String::from("HIGH")
                ]
            }
            .render(),
            "HIGH 100%"
        );
        assert_eq!(
            AudioData::Volume {
                template: String::from("{ICO} {VOL}%"),
                volume: 120,
                icons: vec![
                    String::from("LOW"),
                    String::from("MIDDLE"),
                    String::from("HIGH")
                ]
            }
            .render(),
            "HIGH 120%"
        );
    }
}
