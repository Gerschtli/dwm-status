use super::ConfigEntry;
use super::Data;
use super::FEATURE_NAME;
use error::*;
use feature;
use std::process;

const FILTER: &[char] = &['[', ']', '%'];

pub(super) struct Updater {
    data: Data,
    settings: ConfigEntry,
}

impl Updater {
    pub(super) fn new(data: Data, settings: ConfigEntry) -> Self {
        Self { data, settings }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> Box<&dyn feature::Renderable> {
        Box::new(&self.data)
    }

    fn update(&mut self) -> Result<()> {
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
            self.data.update_mute();
        } else {
            let volume = last
                .get(0)
                .wrap_error(FEATURE_NAME, "no volume part found")?
                .parse()
                .wrap_error(FEATURE_NAME, "volume not parsable")?;

            self.data.update_volume(volume);
        }

        Ok(())
    }
}
