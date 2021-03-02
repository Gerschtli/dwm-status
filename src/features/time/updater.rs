use crate::error::Result;
use crate::feature;
use crate::wrapper::date_time;

use super::Data;

pub(super) struct Updater {
    data: Data,
}

impl Updater {
    pub(super) const fn new(data: Data) -> Self {
        Self { data }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        self.data.update(&date_time::DateTime::now());

        Ok(())
    }
}
