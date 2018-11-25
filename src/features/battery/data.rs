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

/* temporarily disabled because missing mock possibilty in tests
#[cfg(test)]
mod tests {
    use super::*;
    use feature::Renderable;

    macro_rules! map {
        ($($k: expr => $v: expr),* $(,)*) => {{
            let mut map = HashMap::new();
            $( map.insert($k, $v); )*
            map
        }}
    }

    #[test]
    fn test_display_data() {
        let icons = vec![String::from("LOW"), String::from("HIGH")];
        let info1 = BatteryInfo {
            capacity: 0.56,
            estimation: Some(time::Duration::from_secs(600)),
            icons: icons.clone(),
        };
        let info2 = BatteryInfo {
            capacity: 0.75,
            estimation: Some(time::Duration::from_secs(720)),
            icons: icons.clone(),
        };
        let info3 = BatteryInfo {
            capacity: 0.21,
            estimation: Some(time::Duration::from_secs(1510)),
            icons: icons.clone(),
        };

        assert_eq!(
            BatteryData {
                ac_online: true,
                batteries: HashMap::new(),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "no_battery"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: HashMap::new(),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "no_battery"
        );

        assert_eq!(
            BatteryData {
                ac_online: true,
                batteries: map!(String::from("BAT0") => info1.clone()),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "charging HIGH 56% (00:10)"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: map!(String::from("BAT0") => info1.clone()),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "discharging HIGH 56% (00:10)"
        );

        assert_eq!(
            BatteryData {
                ac_online: true,
                batteries: map!(
                    String::from("BAT0") => info1.clone(),
                    String::from("BAT1") => info2.clone(),
                ),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "charging HIGH 56% (00:10)-separator-HIGH 75% (00:12)"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: map!(
                    String::from("BAT0") => info1.clone(),
                    String::from("BAT1") => info2.clone(),
                ),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "discharging HIGH 56% (00:10)-separator-HIGH 75% (00:12)"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: map!(
                    String::from("BAT1") => info2.clone(),
                    String::from("BAT2") => info3.clone(),
                    String::from("BAT0") => info1.clone(),
                ),
                settings: settings::Battery {
                    charging: String::from("charging"),
                    debug: false,
                    discharging: String::from("discharging"),
                    enable_notifier: false,
                    no_battery: String::from("no_battery"),
                    notifier_critical: 1,
                    notifier_levels: vec![1, 2],
                    separator: String::from("-separator-"),
                    icons: icons.clone(),
                },
            }
            .render(),
            "discharging HIGH 56% (00:10)-separator-HIGH 75% (00:12)-separator-LOW 21% (00:25)"
        );
    }

    #[test]
    fn test_display_info() {
        let icons = vec![String::from("LOW"), String::from("HIGH")];
        assert_eq!(
            BatteryInfo {
                capacity: 0.,
                estimation: Some(time::Duration::from_secs(0)),
                icons: icons.clone(),
            }
            .render(),
            "LOW 0% (00:00)"
        );
        assert_eq!(
            BatteryInfo {
                capacity: 0.356,
                estimation: Some(time::Duration::from_secs(11759)),
                icons: icons.clone(),
            }
            .render(),
            "LOW 36% (03:15)"
        );
        assert_eq!(
            BatteryInfo {
                capacity: 0.522,
                estimation: None,
                icons: icons.clone(),
            }
            .render(),
            "HIGH 52%"
        );
    }
}
*/
