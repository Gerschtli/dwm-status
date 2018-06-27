use super::CpuLoadData;
use super::FEATURE_NAME;
use async;
use error::*;
use feature;
use io;
use settings;
use std::sync::mpsc;

const PATH_LOADAVG: &str = "/proc/loadavg";

#[derive(Debug)]
pub struct CpuLoad {
    data: CpuLoadData,
    id: String,
    settings: settings::CpuLoad,
    tx: mpsc::Sender<async::Message>,
}

renderable_impl!(CpuLoad);

impl feature::FeatureConfig for CpuLoad {
    type Settings = settings::CpuLoad;

    fn new(id: String, tx: mpsc::Sender<async::Message>, settings: Self::Settings) -> Result<Self> {
        Ok(CpuLoad {
            data: CpuLoadData {
                one: 0.,
                five: 0.,
                fifteen: 0.,
                template: settings.template.clone(),
            },
            id,
            settings,
            tx,
        })
    }
}

impl feature::Feature for CpuLoad {
    feature_default!();

    fn init_notifier(&self) -> Result<()> {
        async::send_message_interval(
            FEATURE_NAME,
            self.id.clone(),
            self.tx.clone(),
            self.settings.update_interval,
            None,
        );

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let content = io::read_file(PATH_LOADAVG)
            .wrap_error(FEATURE_NAME, &format!("failed to read {}", PATH_LOADAVG))?;

        let mut iterator = content.split_whitespace().into_iter();

        self.data = CpuLoadData {
            one: convert_to_float(iterator.next())?,
            five: convert_to_float(iterator.next())?,
            fifteen: convert_to_float(iterator.next())?,
            template: self.settings.template.clone(),
        };

        Ok(())
    }
}

fn convert_to_float(data: Option<&str>) -> Result<f32> {
    data.wrap_error(FEATURE_NAME, "no data found")?
        .parse()
        .wrap_error(FEATURE_NAME, "could not convert to float")
}
