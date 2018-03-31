use feature;

#[derive(Debug)]
pub struct BacklightData(pub f32);

impl feature::Renderable for BacklightData {
    fn render(&self) -> String {
        format!("L {:.0}%", self.0 * 100.)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feature::Renderable;

    #[test]
    fn test_display() {
        assert_eq!(BacklightData(0.).render(), "L 0%");
        assert_eq!(BacklightData(0.15).render(), "L 15%");
        assert_eq!(BacklightData(0.356).render(), "L 36%");
    }
}
