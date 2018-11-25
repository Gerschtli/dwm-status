#[derive(Debug)]
pub struct Match<'a> {
    pub interface: &'static str,
    pub member: Option<&'static str>,
    pub path: &'a str,
}

impl<'a> Match<'a> {
    pub fn build(self) -> String {
        let member = if let Some(ref member) = self.member {
            format!(",member='{}'", member)
        } else {
            String::new()
        };

        format!(
            "type='signal',path='{}',interface='{}'{}",
            self.path, self.interface, member
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        assert_eq!(
            Match {
                interface: "org.freedesktop.DBus.Properties",
                member: Some("DeviceAdded"),
                path: "/org/freedesktop/UPower",
            }
            .build(),
            "type='signal',path='/org/freedesktop/UPower',interface='org.freedesktop.DBus.\
             Properties',member='DeviceAdded'"
        );
    }

    #[test]
    fn test_match_without_member() {
        assert_eq!(
            Match {
                interface: "org.freedesktop.UPower",
                member: None,
                path: "/org/freedesktop/UPower/devices/battery_BAT0",
            }
            .build(),
            "type='signal',path='/org/freedesktop/UPower/devices/battery_BAT0',interface='org.\
             freedesktop.UPower'"
        );
    }
}
