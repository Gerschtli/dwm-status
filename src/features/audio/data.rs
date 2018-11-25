use feature;
use settings;
use utils::icon_by_percentage;

#[derive(Debug)]
pub enum AudioData {
    Mute,
    Volume(u32),
}

impl feature::Renderable for AudioData {
    fn render(&self, settings: &settings::Settings) -> String {
        match *self {
            AudioData::Mute => settings.audio.mute.clone(),
            AudioData::Volume(volume) => {
                let mut rendered = settings
                    .audio
                    .template
                    .replace("{VOL}", &volume.to_string());

                if let Some(icon) = icon_by_percentage(&settings.audio.icons, volume) {
                    rendered = rendered.replace("{ICO}", icon);
                }

                rendered
            },
        }
    }
}

/* temporarily disabled because missing mock possibilty in tests
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
*/
