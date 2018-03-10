use super::AudioData;
use async;
use error::*;
use feature;
use std::process;
use std::sync::mpsc;
use std::time;

const FILTER: &[char] = &['[', ']', '%'];

#[derive(Debug)]
pub struct Audio {
    data: AudioData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Audio {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Audio {
            data: AudioData::Mute,
            id,
            tx,
        })
    }
}

impl feature::Feature for Audio {
    fn id(&self) -> &str {
        &self.id
    }

    fn init_notifier(&self) -> Result<()> {
        async::schedule_update(
            "audio".to_owned(),
            self.id.to_owned(),
            time::Duration::from_secs(60),
            self.tx.clone(),
        )
    }

    fn render(&self) -> String {
        format!("{}", self.data).clone()
    }

    fn update(&mut self) -> Result<()> {
        // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/sound.rs
        let output = process::Command::new("amixer")
            .arg("get")
            .arg("Master")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
            .wrap_error("audio", "getting amixer info failed")?;

        let last_line = &output
            .lines()
            .into_iter()
            .last()
            .wrap_error("audio", "empty amixer output")?;

        let last = last_line
            .split_whitespace()
            .into_iter()
            .filter(|x| x.starts_with('[') && !x.contains("dB"))
            .map(|s| s.trim_matches(FILTER))
            .collect::<Vec<&str>>();

        if last.get(1).map(|muted| *muted == "off").unwrap_or(false) {
            self.data = AudioData::Mute;
            return Ok(());
        }

        let volume = last.get(0)
            .wrap_error("audio", "no volume part found")?
            .parse::<u32>()
            .wrap_error("audio", "volume not parsable")?;

        self.data = AudioData::Volume(volume);
        Ok(())
    }
}
