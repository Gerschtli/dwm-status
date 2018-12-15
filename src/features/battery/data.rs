use super::fmt_capacity;
use super::fmt_time;
use feature;
use settings;
use std::collections::HashMap;
use std::time;
use utils::icon_by_float;

#[derive(Clone, Debug)]
pub struct BatteryInfo {
    pub capacity: f32,
    pub estimation: Option<time::Duration>,
}

impl feature::Renderable for BatteryInfo {
    fn render(&self, settings: &settings::Settings) -> String {
        let mut rendered = String::with_capacity(16);

        if let Some(icon) = icon_by_float(&settings.battery.icons, self.capacity) {
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
pub struct BatteryData {
    pub ac_online: bool,
    pub batteries: HashMap<String, BatteryInfo>,
}

impl feature::Renderable for BatteryData {
    fn render(&self, settings: &settings::Settings) -> String {
        if self.batteries.is_empty() {
            return settings.battery.no_battery.clone();
        }

        let mut keys = self.batteries.keys().collect::<Vec<_>>();
        keys.sort();
        let batteries = keys
            .into_iter()
            .map(|key| self.batteries[key].render(settings))
            .collect::<Vec<_>>()
            .join(&settings.battery.separator);

        format!(
            "{} {}",
            if self.ac_online {
                &settings.battery.charging
            } else {
                &settings.battery.discharging
            },
            batteries
        )
    }
}
