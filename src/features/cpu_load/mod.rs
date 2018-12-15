mod data;
mod feature;

pub(self) use self::data::CpuLoadData;
pub(crate) use self::feature::CpuLoad;

pub(self) const FEATURE_NAME: &str = "cpu_load";
