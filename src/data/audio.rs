use std::fmt;
use std::process::Command;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use super::{Feature, Message};

const FILTER: &[char] = &['[', ']', '%'];

#[derive(Debug)]
pub enum Audio {
    Mute,
    Volume(u32),
}

impl fmt::Display for Audio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Audio::Mute           => write!(f, "MUTE"),
            Audio::Volume(volume) => write!(f, "S {}%", volume),
        }
    }
}

impl Feature for Audio {
    fn init() -> Self {
        // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/sound.rs
        let output = Command::new("amixer")
            .arg("get")
            .arg("Master")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
            .unwrap();

        let last_line = &output
            .lines()
            .into_iter()
            .last()
            .unwrap();

        let last = last_line
            .split_whitespace()
            .into_iter()
            .filter(|x| x.starts_with('[') && !x.contains("dB"))
            .map(|s| s.trim_matches(FILTER))
            .collect::<Vec<&str>>();

        if last.get(1).map(|muted| *muted == "off").unwrap_or(false) {
            return Audio::Mute;
        }

        let volume = last.get(0)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Audio::Volume(volume)
    }

    fn wait_for_update(tx: &Sender<Message>) {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Message::Audio(Self::init());
            tx.send(message).unwrap();
        }
    }
}
