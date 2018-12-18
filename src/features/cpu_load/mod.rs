mod data;
mod feature;

pub(self) use self::data::CpuLoadData;
pub(super) use self::feature::Feature;

pub(super) const FEATURE_NAME: &str = "cpu_load";
