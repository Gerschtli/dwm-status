use crate::error::Result;
use crate::feature;
use crate::wrapper::battery::all_batteries;

use super::BatteryNotifier;
use super::Data;

pub(super) struct Updater {
    data: Data,
    notifier: BatteryNotifier,
}

impl Updater {
    pub(super) const fn new(data: Data, notifier: BatteryNotifier) -> Self {
        Self { data, notifier }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        let batteries = all_batteries()?;

        self.notifier.update(&batteries);
        self.data.update(&batteries);
        self.data.update_with_status2d();

        Ok(())
    }
}
