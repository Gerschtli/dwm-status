use std::fmt;

#[derive(Debug)]
pub struct Backlight(pub f32);

impl fmt::Display for Backlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L {:.0}%", self.0 * 100.0)
    }
}
