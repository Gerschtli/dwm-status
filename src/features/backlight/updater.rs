use crate::error::Result;
use crate::feature;

use super::BacklightDevice;
use super::Data;

pub(super) struct Updater {
    data: Data,
    device: BacklightDevice,
}

impl Updater {
    pub(super) const fn new(data: Data, device: BacklightDevice) -> Self {
        Self { data, device }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        self.data.update(self.device.value()?);

        self.data.with_status2d();

        Ok(())
    }
}
