use super::RenderConfig;
use feature;
use utils::icon_by_percentage;

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

    pub(super) fn update_mute(&mut self) {
        self.cache = self.config.mute.clone()
    }

    pub(super) fn update_volume(&mut self, volume: u32) {
        let mut rendered = self
            .config
            .template
            .replace("{VOL}", &format!("{}", volume));

        if let Some(icon) = icon_by_percentage(&self.config.icons, volume) {
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
