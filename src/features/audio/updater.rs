use crate::error::Result;
use crate::error::WrapErrorExt;
use crate::feature;
use crate::features::audio::Backend;
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

    fn update_alsa(&mut self) -> Result<()> {
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
                .get(0)
                .wrap_error(FEATURE_NAME, "no volume part found")?
                .parse()
                .wrap_error(FEATURE_NAME, "volume not parsable")?;

            self.data.update_volume(volume);
        }

        Ok(())
    }

    fn update_pulseaudio(&mut self) -> Result<()> {
        let mute = process::Command::new("pulsemixer", &["--get-mute"])
            .output()
            .wrap_error(FEATURE_NAME, "pulsemixer --get-mute failed")?
            .parse::<u8>()
            .wrap_error(FEATURE_NAME, "mute status could not be read")?;

        // mute is either 1 or 0
        if mute == 1 {
            self.data.update_mute();
        } else {
            let volume = process::Command::new("pulsemixer", &["--get-volume"])
                .output()
                .wrap_error(FEATURE_NAME, "pulsemixer --get-volume failed")?
                .split_whitespace()
                .next()
                .wrap_error(FEATURE_NAME, "current volume could not be read")?
                .parse::<u32>()
                .wrap_error(FEATURE_NAME, "current volume could not be read")?;

            self.data.update_volume(volume);
        }

        Ok(())
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        match self.settings.backend {
            Backend::Alsa => self.update_alsa(),
            Backend::Pulseaudio => self.update_pulseaudio(),
        }
    }
}
