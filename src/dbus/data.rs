macro_rules! enum_string {
    ($name:ident; $($key:ident = $value:expr,)*) => {
        pub enum $name {
            $($key,)*
        }

        impl $name {
            pub fn value(&self) -> String {
                match *self {
                    $($name::$key => String::from($value),)*
                }
            }
        }
    };
}

enum_string!(Interface;
    DBUS_PROPERTIES = "org.freedesktop.DBus.Properties",
    UPOWER = "org.freedesktop.UPower",
);

enum_string!(Member;
    DEVICE_ADDED = "DeviceAdded",
    ENUMERATE_DEVICES = "EnumerateDevices",
    PROPERTIES_CHANGED = "PropertiesChanged",
);

pub struct Match<'a> {
    pub interface: Interface,
    pub member: Option<Member>,
    pub path: &'a str,
}

impl<'a> Match<'a> {
    pub fn build(self) -> String {
        let member = if let Some(ref member) = self.member {
            format!(",member='{}'", member.value())
        } else {
            String::from("")
        };

        format!(
            "type='signal',path='{}',interface='{}'{}",
            self.path,
            self.interface.value(),
            member
        )
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_interface() {
        assert_eq!(
            Interface::DBUS_PROPERTIES.value(),
            "org.freedesktop.DBus.Properties"
        );
        assert_eq!(Interface::UPOWER.value(), "org.freedesktop.UPower");
    }

    #[test]
    fn test_member() {
        assert_eq!(Member::DEVICE_ADDED.value(), "DeviceAdded");
        assert_eq!(Member::ENUMERATE_DEVICES.value(), "EnumerateDevices");
        assert_eq!(Member::PROPERTIES_CHANGED.value(), "PropertiesChanged");
    }

    #[test]
    fn test_match() {
        assert_eq!(
            Match {
                interface: Interface::DBUS_PROPERTIES,
                member: Some(Member::DEVICE_ADDED),
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
                interface: Interface::UPOWER,
                member: None,
                path: "/org/freedesktop/UPower/devices/battery_BAT0",
            }
            .build(),
            "type='signal',path='/org/freedesktop/UPower/devices/battery_BAT0',interface='org.\
             freedesktop.UPower'"
        );
    }
}
