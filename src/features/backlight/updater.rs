use super::BacklightDevice;
use super::Data;
use error::*;
use feature;

pub(super) struct Updater {
    data: Data,
    device: BacklightDevice,
}

impl Updater {
    pub(super) fn new(data: Data, device: BacklightDevice) -> Self {
        Self { data, device }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> Box<&dyn feature::Renderable> {
        Box::new(&self.data)
    }

    fn update(&mut self) -> Result<()> {
        self.data.update(self.device.value()?);

        Ok(())
    }
}
