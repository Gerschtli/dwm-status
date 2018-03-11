use std::fmt;

#[derive(Debug)]
pub struct BacklightData(pub f32);

impl fmt::Display for BacklightData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L {:.0}%", self.0 * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", BacklightData(0.)), "L 0%");
        assert_eq!(format!("{}", BacklightData(0.15)), "L 15%");
        assert_eq!(format!("{}", BacklightData(0.356)), "L 36%");
    }
}
