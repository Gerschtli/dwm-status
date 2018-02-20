use std::fmt;
use std::sync::mpsc::Sender;

use super::{Feature, Message};

#[derive(Debug)]
pub struct Backlight(u8);

impl fmt::Display for Backlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L: {}%", self.0)
    }
}

impl Feature for Backlight {
    fn init() -> Self {
        // TODO: replace mock
        Backlight(20)
    }

    fn wait_for_update(tx: &Sender<Message>) {
        // TODO
    }
}
