use super::CpuLoadData;
use super::FEATURE_NAME;
use communication;
use error::*;
use feature;
use io;
use settings;
use std::sync::mpsc;
use std::thread;
use std::time;

const PATH_LOADAVG: &str = "/proc/loadavg";

#[derive(Debug)]
pub(crate) struct Feature {
    id: usize,
    settings: settings::CpuLoad,
    tx: mpsc::Sender<communication::Message>,
}

impl feature::FeatureConfig for Feature {
    type Settings = settings::CpuLoad;

    fn new(
        id: usize,
        tx: mpsc::Sender<communication::Message>,
        settings: Self::Settings,
    ) -> Result<Self> {
        Ok(Self { id, settings, tx })
    }
}

impl feature::Feature for Feature {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        let id = self.id;
        let tx = self.tx.clone();
        let update_interval = self.settings.update_interval;

        thread::spawn(move || loop {
            thread::sleep(time::Duration::from_secs(update_interval));

            communication::send_message(FEATURE_NAME, id, &tx);
        });

        Ok(())
    }

    fn update(&mut self) -> Result<Box<dyn feature::Renderable>> {
        let content = io::read_file(PATH_LOADAVG)
            .wrap_error(FEATURE_NAME, &format!("failed to read {}", PATH_LOADAVG))?;

        let mut iterator = content.split_whitespace();

        let one = convert_to_float(iterator.next())?;
        let five = convert_to_float(iterator.next())?;
        let fifteen = convert_to_float(iterator.next())?;

        Ok(Box::new(CpuLoadData { one, five, fifteen }))
    }
}

fn convert_to_float(data: Option<&str>) -> Result<f32> {
    data.wrap_error(FEATURE_NAME, "no data found")?
        .parse()
        .wrap_error(FEATURE_NAME, "could not convert to float")
}
