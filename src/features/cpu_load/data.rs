use feature;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    template: String,
}

impl Data {
    pub(super) fn new(template: String) -> Self {
        Self {
            cache: String::new(),
            template,
        }
    }

    pub(super) fn update(&mut self, one: f32, five: f32, fifteen: f32) {
        self.cache = self
            .template
            .replace("{CL1}", &format!("{:.2}", one))
            .replace("{CL5}", &format!("{:.2}", five))
            .replace("{CL15}", &format!("{:.2}", fifteen));
    }
}

impl feature::Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}
