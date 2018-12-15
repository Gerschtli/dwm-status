use super::AudioData;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use settings;
use std::io::Read;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time;
use uuid;

const FILTER: &[char] = &['[', ']', '%'];

#[derive(Debug)]
pub(crate) struct Audio {
    id: uuid::Uuid,
    settings: settings::Audio,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Audio {
    type Settings = settings::Audio;

    fn new(
        id: uuid::Uuid,
        tx: mpsc::Sender<async::Message>,
        settings: Self::Settings,
    ) -> Result<Self> {
        Ok(Self { id, settings, tx })
    }
}

impl feature::Feature for Audio {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id;
        let tx = self.tx.clone();

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
                if let Ok(bytes) = monitor.read(&mut buffer) {
                    // reader has reached end-of-life -> thread gets killed
                    if bytes == 0 {
                        break;
                    }

                    async::send_message(FEATURE_NAME, id, &tx);
                }

                // prevent event spamming
                thread::sleep(time::Duration::from_millis(100));
            }
        });

        Ok(())
    }

    fn update(&mut self) -> Result<Box<dyn feature::Renderable>> {
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
            return Ok(Box::new(AudioData::Mute));
        }

        let volume = last
            .get(0)
            .wrap_error(FEATURE_NAME, "no volume part found")?
            .parse()
            .wrap_error(FEATURE_NAME, "volume not parsable")?;

        Ok(Box::new(AudioData::Volume(volume)))
    }
}
