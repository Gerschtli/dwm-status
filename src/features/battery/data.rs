use super::fmt_capacity;
use super::fmt_time;
use super::RenderConfig;
use feature;
use std::collections::HashMap;
use std::time;
use utils::icon_by_percentage;

#[derive(Clone, Debug)]
pub(super) struct BatteryInfo {
    pub(super) capacity: u32,
    pub(super) estimation: Option<time::Duration>,
}

impl BatteryInfo {
    fn render(&self, settings: &RenderConfig) -> String {
        let mut rendered = String::with_capacity(16);

        if let Some(icon) = icon_by_percentage(&settings.icons, self.capacity) {
            rendered.push_str(&format!("{} ", icon));
        }

        rendered.push_str(&fmt_capacity(self.capacity));

        if let Some(ref estimation) = self.estimation {
            rendered.push_str(&format!(" ({})", fmt_time(estimation)));
        }

        rendered
    }
}

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

    pub(super) fn update(&mut self, ac_online: bool, batteries: &HashMap<String, BatteryInfo>) {
        if batteries.is_empty() {
            self.cache = self.config.no_battery.clone();
            return;
        }

        let mut keys = batteries.keys().collect::<Vec<_>>();
        keys.sort();
        let rendered = keys
            .into_iter()
            .map(|key| batteries[key].render(&self.config))
            .collect::<Vec<_>>()
            .join(&self.config.separator);

        self.cache = format!(
            "{} {}",
            if ac_online {
                &self.config.charging
            } else {
                &self.config.discharging
            },
            rendered
        );
    }
}

impl feature::Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}
