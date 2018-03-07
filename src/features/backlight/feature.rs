use super::BacklightData;
use async;
use error::*;
use feature;
use io;
use std::sync::mpsc;
use std::time;
use uuid;

#[derive(Debug)]
pub struct Backlight {
    data: BacklightData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Backlight {
    fn new(tx: &mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Backlight {
            data: BacklightData(0.),
            id: uuid::Uuid::new_v4().simple().to_string(),
            tx: tx.clone(),
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
        format!("{}", self.data).clone()
    }

    fn update(&mut self) -> Result<()> {
        let max = io::value_from_file::<i32>("/sys/class/backlight/intel_backlight/max_brightness")
            .unwrap();
        let current = io::value_from_file::<i32>(
            "/sys/class/backlight/intel_backlight/actual_brightness",
        ).unwrap();

        self.data = BacklightData(current as f32 / max as f32);

        Ok(())
    }
}
