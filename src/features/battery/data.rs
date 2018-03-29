use std::collections::HashMap;
use std::fmt;
use std::time;

#[derive(Clone, Debug)]
pub struct BatteryInfo {
    pub capacity: f32,
    pub estimation: time::Duration,
}

impl fmt::Display for BatteryInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let minutes = self.estimation.as_secs() / 60;

        write!(
            f,
            "{:.0}% ({:02}:{:02})",
            self.capacity * 100.,
            minutes / 60,
            minutes % 60,
        )
    }
}

#[derive(Debug)]
pub struct BatteryData {
    pub ac_online: bool,
    pub batteries: HashMap<String, BatteryInfo>,
}

impl fmt::Display for BatteryData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.batteries.is_empty() {
            return write!(f, "NO BATT");
        }

        let mut keys = self.batteries.keys().collect::<Vec<_>>();
        keys.sort();
        let batteries = keys.into_iter()
            .map(|key| format!("{}", &self.batteries[key]))
            .collect::<Vec<_>>()
            .join(" · ");

        write!(
            f,
            "{} {}",
            if self.ac_online { '▲' } else { '▼' },
            batteries
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            estimation: time::Duration::from_secs(600),
        };
        let info2 = BatteryInfo {
            capacity: 0.75,
            estimation: time::Duration::from_secs(720),
        };
        let info3 = BatteryInfo {
            capacity: 0.21,
            estimation: time::Duration::from_secs(1510),
        };

        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: true,
                    batteries: HashMap::new(),
                }
            ),
            "NO BATT"
        );
        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: false,
                    batteries: HashMap::new(),
                },
            ),
            "NO BATT"
        );

        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: true,
                    batteries: map!("BAT0".to_owned() => info1.clone()),
                }
            ),
            "▲ 56% (00:10)"
        );
        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: false,
                    batteries: map!("BAT0".to_owned() => info1.clone()),
                },
            ),
            "▼ 56% (00:10)"
        );

        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: true,
                    batteries: map!(
                        "BAT0".to_owned() => info1.clone(),
                        "BAT1".to_owned() => info2.clone(),
                    ),
                }
            ),
            "▲ 56% (00:10) · 75% (00:12)"
        );
        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: false,
                    batteries: map!(
                        "BAT0".to_owned() => info1.clone(),
                        "BAT1".to_owned() => info2.clone(),
                    ),
                }
            ),
            "▼ 56% (00:10) · 75% (00:12)"
        );
        assert_eq!(
            format!(
                "{}",
                BatteryData {
                    ac_online: false,
                    batteries: map!(
                        "BAT1".to_owned() => info2.clone(),
                        "BAT2".to_owned() => info3.clone(),
                        "BAT0".to_owned() => info1.clone(),
                    ),
                }
            ),
            "▼ 56% (00:10) · 75% (00:12) · 21% (00:25)"
        );
    }

    #[test]
    fn test_display_info() {
        assert_eq!(
            format!(
                "{}",
                BatteryInfo {
                    capacity: 0.,
                    estimation: time::Duration::from_secs(0),
                }
            ),
            "0% (00:00)"
        );
        assert_eq!(
            format!(
                "{}",
                BatteryInfo {
                    capacity: 0.356,
                    estimation: time::Duration::from_secs(3 * 60 * 60 + 15 * 60 + 59),
                }
            ),
            "36% (03:15)"
        );
    }
}
