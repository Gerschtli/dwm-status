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
            self.capacity * 100.0,
            minutes / 60,
            minutes % 60,
        )
    }
}

#[derive(Debug)]
pub enum BatteryData {
    Charging(BatteryInfo),
    Discharging(BatteryInfo),
    Full,
    NoBattery,
}

impl fmt::Display for BatteryData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatteryData::Charging(ref info) => write!(f, "+ {}", info),
            BatteryData::Discharging(ref info) => write!(f, "- {}", info),
            BatteryData::Full => write!(f, "= 100%"),
            BatteryData::NoBattery => write!(f, "NO BATT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_data() {
        let info = BatteryInfo {
            capacity: 0.56,
            estimation: time::Duration::from_secs(600),
        };

        assert_eq!(
            format!("{}", BatteryData::Charging(info.clone())),
            "+ 56% (00:10)"
        );
        assert_eq!(
            format!("{}", BatteryData::Discharging(info)),
            "- 56% (00:10)"
        );
        assert_eq!(format!("{}", BatteryData::Full), "= 100%");
        assert_eq!(format!("{}", BatteryData::NoBattery), "NO BATT");
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
