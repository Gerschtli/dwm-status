use super::fmt_capacity;
use super::fmt_time;
use feature;
use std::collections::HashMap;
use std::time;

#[derive(Clone, Debug)]
pub struct BatteryInfo {
    pub capacity: f32,
    pub estimation: Option<time::Duration>,
}

impl feature::Renderable for BatteryInfo {
    fn render(&self) -> String {
        let mut rendered = fmt_capacity(self.capacity);

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
    fn render(&self) -> String {
        if self.batteries.is_empty() {
            return String::from("NO BATT");
        }

        let mut keys = self.batteries.keys().collect::<Vec<_>>();
        keys.sort();
        let batteries = keys
            .into_iter()
            .map(|key| self.batteries[key].render())
            .collect::<Vec<_>>()
            .join(" · ");

        format!(
            "{} {}",
            if self.ac_online { '▲' } else { '▼' },
            batteries
        )
    }
}

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
        let info1 = BatteryInfo {
            capacity: 0.56,
            estimation: Some(time::Duration::from_secs(600)),
        };
        let info2 = BatteryInfo {
            capacity: 0.75,
            estimation: Some(time::Duration::from_secs(720)),
        };
        let info3 = BatteryInfo {
            capacity: 0.21,
            estimation: Some(time::Duration::from_secs(1510)),
        };

        assert_eq!(
            BatteryData {
                ac_online: true,
                batteries: HashMap::new(),
            }.render(),
            "NO BATT"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: HashMap::new(),
            }.render(),
            "NO BATT"
        );

        assert_eq!(
            BatteryData {
                ac_online: true,
                batteries: map!(String::from("BAT0") => info1.clone()),
            }.render(),
            "▲ 56% (00:10)"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: map!(String::from("BAT0") => info1.clone()),
            }.render(),
            "▼ 56% (00:10)"
        );

        assert_eq!(
            BatteryData {
                ac_online: true,
                batteries: map!(
                    String::from("BAT0") => info1.clone(),
                    String::from("BAT1") => info2.clone(),
                ),
            }.render(),
            "▲ 56% (00:10) · 75% (00:12)"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: map!(
                    String::from("BAT0") => info1.clone(),
                    String::from("BAT1") => info2.clone(),
                ),
            }.render(),
            "▼ 56% (00:10) · 75% (00:12)"
        );
        assert_eq!(
            BatteryData {
                ac_online: false,
                batteries: map!(
                    String::from("BAT1") => info2.clone(),
                    String::from("BAT2") => info3.clone(),
                    String::from("BAT0") => info1.clone(),
                ),
            }.render(),
            "▼ 56% (00:10) · 75% (00:12) · 21% (00:25)"
        );
    }

    #[test]
    fn test_display_info() {
        assert_eq!(
            BatteryInfo {
                capacity: 0.,
                estimation: Some(time::Duration::from_secs(0)),
            }.render(),
            "0% (00:00)"
        );
        assert_eq!(
            BatteryInfo {
                capacity: 0.356,
                estimation: Some(time::Duration::from_secs(11759)),
            }.render(),
            "36% (03:15)"
        );
        assert_eq!(
            BatteryInfo {
                capacity: 0.356,
                estimation: None,
            }.render(),
            "36%"
        );
    }
}
