use super::TimeData;
use async;
use chrono;
use error::*;
use feature;
use std::sync::mpsc;
use std::time;

#[derive(Debug)]
pub struct Time {
    data: TimeData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Time {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Time {
            data: TimeData(chrono::Local::now()),
            id,
            tx,
        })
    }
}

impl feature::Feature for Time {
    fn id(&self) -> &str {
        &self.id
    }

    fn init_notifier(&self) -> Result<()> {
        async::schedule_update(
            "time".to_owned(),
            self.id.to_owned(),
            time::Duration::from_secs(60),
            self.tx.clone(),
        )
    }

    fn render(&self) -> String {
        format!("{}", self.data)
    }

    fn update(&mut self) -> Result<()> {
        self.data = TimeData(chrono::Local::now());
        Ok(())
    }
}
