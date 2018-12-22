use super::RenderConfig;
use feature;
use utils::icon_by_percentage;

#[derive(Debug)]
pub(super) struct Data {
    cache: String,
    config: RenderConfig,
}

impl Data {
    pub(super) fn new(config: RenderConfig) -> Self {
        Self {
            cache: String::new(),
            config,
        }
    }

    pub(super) fn update(&mut self, value: u32) {
        let mut rendered = self.config.template.replace("{BL}", &format!("{}", value));

        if let Some(icon) = icon_by_percentage(&self.config.icons, value) {
            rendered = rendered.replace("{ICO}", icon);
        }

        self.cache = rendered;
    }
}

impl feature::Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}
