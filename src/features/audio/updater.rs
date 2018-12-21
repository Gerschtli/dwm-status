use super::Data;
use super::FEATURE_NAME;
use error::*;
use feature::Updatable;
use settings;
use std::process;

const FILTER: &[char] = &['[', ']', '%'];

pub(super) struct Updater {
    settings: settings::Audio,
}

impl Updater {
    pub(super) fn new(settings: settings::Audio) -> Self {
        Self { settings }
    }
}

impl Updatable for Updater {
    type Data = Data;

    fn update(&mut self) -> Result<Self::Data> {
        // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/sound.rs
        let output = process::Command::new("amixer")
            .arg("get")
            .arg(&self.settings.control)
            .output()
            .map(|o| String::from(String::from_utf8_lossy(&o.stdout).trim()))
            .wrap_error(FEATURE_NAME, "getting amixer info failed")?;

        let last_line = &output
            .lines()
            .last()
            .wrap_error(FEATURE_NAME, "empty amixer output")?;

        let last = last_line
            .split_whitespace()
            .filter(|x| x.starts_with('[') && !x.contains("dB"))
            .map(|s| s.trim_matches(FILTER))
            .collect::<Vec<&str>>();

        if last.get(1).map_or(false, |muted| *muted == "off") {
            return Ok(Data::Mute);
        }

        let volume = last
            .get(0)
            .wrap_error(FEATURE_NAME, "no volume part found")?
            .parse()
            .wrap_error(FEATURE_NAME, "volume not parsable")?;

        Ok(Data::Volume(volume))
    }
}
