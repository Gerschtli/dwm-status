use feature;

#[derive(Debug)]
pub struct BacklightData {
    pub template: String,
    pub value: f32,
}

impl feature::Renderable for BacklightData {
    fn render(&self) -> String {
        self.template
            .replace("{BL}", &format!("{:.0}", self.value * 100.))
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
                value: 0.
            }.render(),
            "L 0%"
        );
        assert_eq!(
            BacklightData {
                template: String::from("XX {BL}% {BL}"),
                value: 0.15
            }.render(),
            "XX 15% 15"
        );
        assert_eq!(
            BacklightData {
                template: String::from("L {BL}%"),
                value: 0.356
            }.render(),
            "L 36%"
        );
        assert_eq!(
            BacklightData {
                template: String::from("L {BF}%"),
                value: 0.356
            }.render(),
            "L {BF}%"
        );
    }
}
