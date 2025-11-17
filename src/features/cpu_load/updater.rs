use crate::error::Result;
use crate::error::WrapErrorExt;
use crate::feature;
use crate::wrapper::file;

use super::Data;
use super::FEATURE_NAME;

use regex::Regex;

const PATH_LOADAVG: &str = "/proc/loadavg";
const PATH_NPROC: &str = "/proc/cpuinfo";

const NPROC_REGEX: &str = r"processor\s+: \d+";

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
        let content = file::read(PATH_LOADAVG)
            .wrap_error(FEATURE_NAME, format!("failed to read {}", PATH_LOADAVG))?;

        let mut iterator = content.split_whitespace();

        let one = convert_to_float(iterator.next())?;
        let five = convert_to_float(iterator.next())?;
        let fifteen = convert_to_float(iterator.next())?;

        let nproc_content = file::read(PATH_NPROC)
            .wrap_error(FEATURE_NAME, format!("failed to read {}", PATH_NPROC))?;

        let re = Regex::new(NPROC_REGEX).unwrap();
        let nproc = re.find_iter(&nproc_content).count() as u32;

        self.data.update(one, five, fifteen, nproc);

        Ok(())
    }
}

fn convert_to_float(data: Option<&str>) -> Result<f32> {
    data.wrap_error(FEATURE_NAME, "no data found")?
        .parse()
        .wrap_error(FEATURE_NAME, "could not convert to float")
}
