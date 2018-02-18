use std::fmt;

use super::{Init, OptionalFeature};

#[derive(Debug)]
pub struct Backlight(u8);

impl fmt::Display for Backlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L: {}%", self.0)
    }
}

impl Init for Backlight {
    fn init() -> Self {
        // TODO: replace mock
        Backlight(20)
    }
}

impl OptionalFeature for Backlight {
    fn has_feature() -> bool {
        // TODO: replace mock
        true
    }
}
