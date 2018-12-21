use super::Data;
use chrono;
use error::*;
use feature::Updatable;

pub(super) struct Updater;

impl Updatable for Updater {
    type Data = Data;

    fn update(&mut self) -> Result<Self::Data> {
        Ok(Data(chrono::Local::now()))
    }
}
