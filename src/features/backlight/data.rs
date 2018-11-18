use feature;
use utils::icon_by_percentage;

#[derive(Debug)]
pub struct BacklightData {
    pub template: String,
    pub value: f32,
    pub icons: Vec<String>,
}

impl feature::Renderable for BacklightData {
    fn render(&self) -> String {
        let value = self.value * 100.;
        let mut rendered = self.template.replace("{BL}", &format!("{:.0}", value));
        let icon_optional = icon_by_percentage(&self.icons, value as u32);
        if let Some(icon) = icon_optional {
            rendered = rendered.replace("{ICO}", icon);
        }
        rendered
    }
}

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
