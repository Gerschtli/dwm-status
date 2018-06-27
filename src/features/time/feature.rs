use super::TimeData;
use super::FEATURE_NAME;
use async;
use chrono;
use error::*;
use feature;
use settings;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Time {
    data: TimeData,
    id: String,
    settings: settings::Time,
    tx: mpsc::Sender<async::Message>,
}

renderable_impl!(Time);

impl feature::FeatureConfig for Time {
    type Settings = settings::Time;

    fn new(id: String, tx: mpsc::Sender<async::Message>, settings: Self::Settings) -> Result<Self> {
        Ok(Time {
            data: TimeData {
                format: settings.format.clone(),
                time: chrono::Local::now(),
            },
            id,
            settings,
            tx,
        })
    }
}

impl feature::Feature for Time {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let (interval, delay) = if self.settings.update_seconds {
            (1, None)
        } else {
            let seconds_string = format!("{}", chrono::Local::now().format("%S"));
            let seconds = seconds_string
                .parse::<u64>()
                .wrap_error(FEATURE_NAME, "could extract initial delay seconds")?;

            (60, Some(60 - seconds))
        };

        async::send_message_interval(
            FEATURE_NAME,
            self.id.clone(),
            self.tx.clone(),
            interval,
            delay,
        );

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.data = TimeData {
            format: self.settings.format.clone(),
            time: chrono::Local::now(),
        };

        Ok(())
    }
}
