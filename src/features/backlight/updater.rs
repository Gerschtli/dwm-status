use super::BacklightDevice;
use super::Data;
use error::*;
use feature::Updatable;

pub(super) struct Updater {
    device: BacklightDevice,
}

impl Updater {
    pub(super) fn new(device: BacklightDevice) -> Self {
        Self { device }
    }
}

impl Updatable for Updater {
    type Data = Data;

    fn update(&mut self) -> Result<Self::Data> {
        Ok(Data(self.device.value()?))
    }
}
