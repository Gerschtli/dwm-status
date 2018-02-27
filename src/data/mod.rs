pub mod audio;
pub mod backlight;
pub mod battery;
pub mod time;

use std::fmt;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use io::render_status;

use self::audio::Audio;
use self::backlight::Backlight;
use self::battery::Battery;
use self::time::Time;

const STATUS_SEPARATOR: &str = " / ";

pub trait Feature {
    fn is_enabled() -> bool {
        true
    }

    fn init() -> Self;

    fn wait_for_update(tx: &Sender<Message>);
}

#[derive(Debug)]
pub enum Message {
    Audio(Audio),
    Backlight(Backlight),
    Battery(Battery),
    Time(Time),
}

#[derive(Debug)]
pub struct SystemInfo {
    audio: Option<Audio>,
    backlight: Option<Backlight>,
    battery: Option<Battery>,
    time: Option<Time>,
}

impl fmt::Display for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn write_feature<T: Feature + fmt::Display>(
            f: &mut fmt::Formatter,
            feature: &Option<T>,
            with_separator: bool
        ) -> fmt::Result {
            if let &Some(ref value) = feature {
                let sep = if with_separator { STATUS_SEPARATOR } else { "" };
                try!(write!(f, "{}{}", value, sep));
            }

            Ok(())
        }

        try!(write_feature(f, &self.backlight, true));
        try!(write_feature(f, &self.audio, true));
        try!(write_feature(f, &self.battery, true));
        write_feature(f, &self.time, false)
    }
}


impl SystemInfo {
    pub fn init() -> Self {
        fn init_feature<T: Feature>() -> Option<T> {
            if T::is_enabled() {
                Some(T::init())
            } else {
                None
            }
        }

        SystemInfo {
            audio:     init_feature(),
            backlight: init_feature(),
            battery:   init_feature(),
            time:      init_feature(),
        }
    }

    pub fn listen(&mut self) {
        fn listen_for_changes<T: Feature>(tx: Sender<Message>, feature: &Option<T>) {
            if feature.is_some() {
                thread::spawn(move || {
                    T::wait_for_update(&tx);
                });
            }
        }

        let (tx, rx) = channel();

        listen_for_changes(tx.clone(), &self.audio);
        listen_for_changes(tx.clone(), &self.backlight);
        listen_for_changes(tx.clone(), &self.battery);
        listen_for_changes(tx.clone(), &self.time);

        for message in rx {
            println!("Message: {:?}", message);

            match message {
                Message::Audio(audio)         => { self.audio     = Some(audio); },
                Message::Backlight(backlight) => { self.backlight = Some(backlight); },
                Message::Battery(battery)     => { self.battery   = Some(battery); },
                Message::Time(time)           => { self.time      = Some(time); },
            }

            self.render();
        }
    }

    pub fn render(&self) {
        println!("{:#?}", &self);
        render_status(&format!("{}", &self));
    }
}
