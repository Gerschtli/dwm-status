pub mod audio;
pub mod backlight;
pub mod battery;
pub mod time;

use std::fmt;

use self::audio::Audio;
use self::backlight::Backlight;
use self::battery::Battery;
use self::time::Time;

const STATUS_SEPARATOR: &str = " / ";

pub trait Init {
    fn init() -> Self;
}

pub trait OptionalFeature {
    fn has_feature() -> bool;
}

#[derive(Debug)]
pub struct SystemInfo {
    audio: Audio,
    backlight: Option<Backlight>,
    battery: Option<Battery>,
    time: Time,
}

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref backlight) = self.backlight {
            try!(write!(f, "{}{}", backlight, STATUS_SEPARATOR));
        }

        try!(write!(f, "{}{}", self.audio, STATUS_SEPARATOR));

        if let Some(ref battery) = self.battery {
            try!(write!(f, "{}{}", battery, STATUS_SEPARATOR));
        }

        write!(f, "{}", self.time)
    }
}

impl Init for SystemInfo {
    fn init() -> Self {
        SystemInfo {
            audio: Audio::init(),
            backlight: if Backlight::has_feature() { Some(Backlight::init()) } else { None },
            battery: if Battery::has_feature() { Some(Battery::init()) } else { None },
            time: Time::init(),
        }
    }
}
