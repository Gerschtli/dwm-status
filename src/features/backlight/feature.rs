use super::BacklightData;
use async;
use error::*;
use feature;
use io;
use std::sync::mpsc;
use std::time;

#[derive(Debug)]
pub struct Backlight {
    data: BacklightData,
    id: String,
    tx: mpsc::Sender<async::Message>,
}

impl feature::FeatureConfig for Backlight {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self> {
        Ok(Backlight {
            data: BacklightData(0.),
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
        format!("{}", self.data).clone()
    }

    fn update(&mut self) -> Result<()> {
        let max = io::read_int_from_file("/sys/class/backlight/intel_backlight/max_brightness")
            .wrap_error("backlight", "error reading max brightness")?;

        let current = io::read_int_from_file(
            "/sys/class/backlight/intel_backlight/actual_brightness",
        ).wrap_error("backlight", "error reading actual brightness")?;

        self.data = BacklightData(current as f32 / max as f32);

        Ok(())
    }
}
