pub(crate) struct Match<'a> {
    interface: &'static str,
    member: Option<&'static str>,
    path: &'a str,
}

impl<'a> Match<'a> {
    pub(crate) fn new<M: Into<Option<&'static str>>>(
        interface: &'static str,
        member: M,
        path: &'a str,
    ) -> Self {
        Self {
            interface,
            member: member.into(),
            path,
        }
    }

    pub(crate) fn build(self) -> String {
        let member = self
            .member
            .map_or_else(String::new, |m| format!(",member='{}'", m));

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
    fn match_build() {
        assert_eq!(
            Match::new(
                "org.freedesktop.DBus.Properties",
                "DeviceAdded",
                "/org/freedesktop/UPower",
            )
            .build(),
            "type='signal',path='/org/freedesktop/UPower',interface='org.freedesktop.DBus.\
             Properties',member='DeviceAdded'"
        );
    }

    #[test]
    fn match_build_without_member() {
        assert_eq!(
            Match::new(
                "org.freedesktop.UPower",
                None,
                "/org/freedesktop/UPower/devices/battery_BAT0",
            )
            .build(),
            "type='signal',path='/org/freedesktop/UPower/devices/battery_BAT0',interface='org.\
             freedesktop.UPower'"
        );
    }
}
