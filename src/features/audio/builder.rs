use std::process::Command;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use error::*;
use features::{Feature, FeatureBuilder};
use super::Audio;

const FILTER: &[char] = &['[', ']', '%'];

pub struct AudioBuilder;

impl FeatureBuilder for AudioBuilder {
    type Data = Audio;

    fn build(&self) -> Result<Self::Data> {
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
            return Ok(Audio::Mute);
        }

        let volume = last.get(0)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        Ok(Audio::Volume(volume))
    }

    fn wait_for_update(&self, tx: &Sender<Feature>) -> Result<()> {
        // TODO: react on system messages
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Feature::Audio(Some(self.build().unwrap()));
            tx.send(message).unwrap();
        }
    }
}
