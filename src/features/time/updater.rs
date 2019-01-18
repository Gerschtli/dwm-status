use super::Data;
use error::*;
use feature;
use wrapper::date_time;

pub(super) struct Updater {
    data: Data,
}

impl Updater {
    pub(super) fn new(data: Data) -> Self {
        Self { data }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> Box<&dyn feature::Renderable> {
        Box::new(&self.data)
    }

    fn update(&mut self) -> Result<()> {
        self.data.update(&date_time::DateTime::now());

        Ok(())
    }
}
