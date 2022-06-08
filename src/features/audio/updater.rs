use crate::error::Result;
use crate::error::WrapErrorExt;
use crate::feature;
use crate::wrapper::process;

use super::ConfigEntry;
use super::Data;
use super::FEATURE_NAME;

const FILTER: &[char] = &['[', ']', '%'];

pub(super) struct Updater {
    data: Data,
    settings: ConfigEntry,
}

impl Updater {
    pub(super) const fn new(data: Data, settings: ConfigEntry) -> Self {
        Self { data, settings }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/sound.rs
        let output = process::Command::new("amixer", &["get", &self.settings.control])
            .output()
            .wrap_error(
                FEATURE_NAME,
                format!(
                    "amixer info for control '{}' could not be fetched",
                    &self.settings.control,
                ),
            )?;

        let last_line = &output
            .lines()
            .last()
            .wrap_error(FEATURE_NAME, "empty amixer output")?;

        let last = last_line
            .split_whitespace()
            .filter(|x| x.starts_with('[') && !x.contains("dB"))
            .map(|s| s.trim_matches(FILTER))
            .collect::<Vec<_>>();

        if last.get(1).map_or(false, |muted| *muted == "off") {
            self.data.update_mute();
        } else {
            let volume = last
                .first()
                .wrap_error(FEATURE_NAME, "no volume part found")?
                .parse()
                .wrap_error(FEATURE_NAME, "volume not parsable")?;

            self.data.update_volume(volume);
        }

        Ok(())
    }
}
