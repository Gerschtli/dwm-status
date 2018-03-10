use super::BacklightData;
use super::BacklightDevice;
use async;
use error::*;
use feature;
use std::sync::mpsc;
use std::time;

#[derive(Debug)]
pub struct Backlight {
    data: BacklightData,
    device: BacklightDevice,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Backlight {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Backlight {
            data: BacklightData(0.),
            device: BacklightDevice::new()?,
            id,
            tx,
        })
    }
}

impl feature::Feature for Backlight {
    fn id(&self) -> &str {
        &self.id
    }

    fn init_notifier(&self) -> Result<()> {
        async::schedule_update(
            "backlight".to_owned(),
            self.id.to_owned(),
            time::Duration::from_secs(60),
            self.tx.clone(),
        )
    }

    fn render(&self) -> String {
        format!("{}", self.data)
    }

    fn update(&mut self) -> Result<()> {
        self.data = BacklightData(self.device.value()?);

        Ok(())
    }
}
