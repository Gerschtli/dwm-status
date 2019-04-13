use super::fmt_capacity;
use super::fmt_time;
use super::RenderConfig;
use crate::feature::Renderable;
use crate::utils::icon_by_percentage;
use std::collections::HashMap;
use std::time;

#[derive(Debug)]
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

impl Renderable for Data {
    fn render(&self) -> &str {
        &self.cache
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;
    #[cfg(feature = "mocking")]
    use mocktopus::mocking::*;

    #[test]
    fn render_with_default() {
        let object = build_object();

        assert_that!(object.render(), is(equal_to("")));
    }

    #[test]
    fn render_when_no_battery() {
        let mut object = build_object();

        object.update(true, &HashMap::new());

        assert_that!(object.render(), is(equal_to("no_battery")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_one_battery_and_charging() {
        let mut counter = 0;
        icon_by_percentage.mock_safe(move |icons, value| {
            counter += 1;
            assert_that!(icons, contains(vec!["icons".to_owned()]).exactly());

            match counter {
                1 => {
                    assert_that!(value, is(equal_to(40)));
                    MockResult::Return(None)
                },
                _ => panic!("icon_by_percentage called to often: {} times", counter),
            }
        });

        let mut batteries = HashMap::new();
        batteries.insert(
            "BAT0".to_owned(),
            BatteryInfo {
                capacity: 40,
                estimation: None,
            },
        );

        let mut object = build_object();

        object.update(true, &batteries);

        assert_that!(object.render(), is(equal_to("charging 40%")));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn render_with_multiple_batteries_and_discharging() {
        let mut counter = 0;
        icon_by_percentage.mock_safe(move |icons, value| {
            counter += 1;
            assert_that!(icons, contains(vec!["icons".to_owned()]).exactly());

            match counter {
                1 => {
                    assert_that!(value, is(equal_to(40)));
                    MockResult::Return(Some("ico40"))
                },
                2 => {
                    assert_that!(value, is(equal_to(70)));
                    MockResult::Return(Some("ico70"))
                },
                _ => panic!("icon_by_percentage called to often: {} times", counter),
            }
        });

        let mut batteries = HashMap::new();
        batteries.insert(
            "BAT1".to_owned(),
            BatteryInfo {
                capacity: 70,
                estimation: Some(time::Duration::from_secs((2 * 60 + 7) * 60)),
            },
        );
        batteries.insert(
            "BAT0".to_owned(),
            BatteryInfo {
                capacity: 40,
                estimation: Some(time::Duration::from_secs(30 * 60)),
            },
        );

        let mut object = build_object();

        object.update(false, &batteries);

        assert_that!(
            object.render(),
            is(equal_to(
                "discharging ico40 40% (00:30) # ico70 70% (02:07)"
            ))
        );
    }

    fn build_object() -> Data {
        Data::new(RenderConfig {
            charging: "charging".to_owned(),
            discharging: "discharging".to_owned(),
            icons: vec!["icons".to_owned()],
            no_battery: "no_battery".to_owned(),
            separator: " # ".to_owned(),
        })
    }
}
