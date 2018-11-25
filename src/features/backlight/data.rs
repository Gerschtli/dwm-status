use feature;
use settings;
use utils::icon_by_float;

#[derive(Debug, Default)]
pub struct BacklightData(pub f32);

impl feature::Renderable for BacklightData {
    fn render(&self, settings: &settings::Settings) -> String {
        let mut rendered = settings
            .backlight
            .template
            .replace("{BL}", &format!("{:.0}", self.0 * 100.));

        if let Some(icon) = icon_by_float(&settings.backlight.icons, self.0) {
            rendered = rendered.replace("{ICO}", icon);
        }

        rendered
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
            BacklightData {
                template: String::from("L {BL}%"),
                value: 0.,
                icons: Vec::<String>::new(),
            }
            .render(),
            "L 0%"
        );
        assert_eq!(
            BacklightData {
                template: String::from("XX {BL}% {BL}"),
                value: 0.15,
                icons: Vec::<String>::new(),
            }
            .render(),
            "XX 15% 15"
        );
        assert_eq!(
            BacklightData {
                template: String::from("L {BL}%"),
                value: 0.356,
                icons: Vec::<String>::new(),
            }
            .render(),
            "L 36%"
        );
        assert_eq!(
            BacklightData {
                template: String::from("L {BF}%"),
                value: 0.356,
                icons: Vec::<String>::new(),
            }
            .render(),
            "L {BF}%"
        );
    }

    #[test]
    fn test_display_with_dynamic_icons() {
        assert_eq!(
            BacklightData {
                template: String::from("{ICO} {BL}%"),
                value: 0.,
                icons: Vec::<String>::new()
            }
            .render(),
            "{ICO} 0%"
        );
        assert_eq!(
            BacklightData {
                template: String::from("{ICO} {BL}%"),
                value: 0.,
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
            BacklightData {
                template: String::from("{ICO} {BL}%"),
                value: 0.65,
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
            BacklightData {
                template: String::from("{ICO} {BL}%"),
                value: 1.,
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
            BacklightData {
                template: String::from("{ICO} {BL}%"),
                value: 1.2,
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
