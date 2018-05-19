use super::AudioData;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use std::io::Read;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time;

const FILTER: &[char] = &['[', ']', '%'];

#[derive(Debug)]
pub struct Audio {
    data: AudioData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

renderable_impl!(Audio);

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
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let tx = self.tx.clone();
        let id = self.id.clone();

        thread::spawn(move || {
            let mut monitor = process::Command::new("sh")
                .args(&["-c", "stdbuf -oL alsactl monitor"])
                .stdout(process::Stdio::piped())
                .spawn()
                .wrap_error_kill(FEATURE_NAME, "failed to start alsactl monitor")
                .stdout
                .wrap_error_kill(FEATURE_NAME, "failed to pipe alsactl monitor output");

            let mut buffer = [0; 1024];
            loop {
                if monitor.read(&mut buffer).is_ok() {
                    async::send_message(FEATURE_NAME, &id, &tx);
                }

                // prevent event spamming
                thread::sleep(time::Duration::from_millis(250));
            }
        });

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        // originally taken from https://github.com/greshake/i3status-rust/blob/master/src/blocks/sound.rs
        let output = process::Command::new("amixer")
            .arg("get")
            .arg("Master")
            .output()
            .map(|o| String::from(String::from_utf8_lossy(&o.stdout).trim()))
            .wrap_error(FEATURE_NAME, "getting amixer info failed")?;

        let last_line = &output
            .lines()
            .into_iter()
            .last()
            .wrap_error(FEATURE_NAME, "empty amixer output")?;

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

        let volume = last
            .get(0)
            .wrap_error(FEATURE_NAME, "no volume part found")?
            .parse::<u32>()
            .wrap_error(FEATURE_NAME, "volume not parsable")?;

        self.data = AudioData::Volume(volume);
        Ok(())
    }
}
