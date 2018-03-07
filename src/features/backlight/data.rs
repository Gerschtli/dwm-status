use std::fmt;

#[derive(Debug)]
pub struct BacklightData(pub f32);

impl fmt::Display for BacklightData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L {:.0}%", self.0 * 100.0)
    }
}
