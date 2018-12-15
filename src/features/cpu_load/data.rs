use feature;
use settings;

#[derive(Debug)]
pub(super) struct CpuLoadData {
    pub(super) one: f32,
    pub(super) five: f32,
    pub(super) fifteen: f32,
}

impl feature::Renderable for CpuLoadData {
    fn render(&self, settings: &settings::Settings) -> String {
        settings
            .cpu_load
            .template
            .replace("{CL1}", &format!("{:.2}", self.one))
            .replace("{CL5}", &format!("{:.2}", self.five))
            .replace("{CL15}", &format!("{:.2}", self.fifteen))
    }
}
