use std::collections::HashSet;

use crate::communication;
use crate::error::Result;
use crate::error::WrapErrorExt;
use crate::wrapper::channel;
use crate::wrapper::dbus;
use crate::wrapper::thread;

use super::FEATURE_NAME;

const INTERFACE_DBUS_PROPERTIES: &str = "org.freedesktop.DBus.Properties";
const INTERFACE_UPOWER: &str = "org.freedesktop.UPower";
const MEMBER_DEVICE_ADDED: &str = "DeviceAdded";
const MEMBER_ENUMERATE_DEVICES: &str = "EnumerateDevices";
const MEMBER_PROPERTIES_CHANGED: &str = "PropertiesChanged";
const PATH_BATTERY_DEVICES_PREFIX: &str = "/org/freedesktop/UPower/devices/battery_";
const PATH_DEVICES_PREFIX: &str = "/org/freedesktop/UPower/devices";
const PATH_UPOWER: &str = "/org/freedesktop/UPower";

pub(super) struct DbusWatcher {
    id: usize,
    sender: channel::Sender<communication::Message>,
}

impl DbusWatcher {
    pub(super) const fn new(id: usize, sender: channel::Sender<communication::Message>) -> Self {
        Self { id, sender }
    }

    fn add_device<'a>(
        &self,
        connection: &dbus::Connection,
        devices: &mut HashSet<dbus::Path<'a>>,
        path: &dbus::Path<'a>,
    ) -> Result<()> {
        let name = self.get_device_name(path)?;

        // ignore line power devices
        if name.starts_with(PATH_DEVICES_PREFIX) || devices.contains(path) {
            return Ok(());
        }

        connection.add_match(dbus::Match::new(
            INTERFACE_DBUS_PROPERTIES,
            MEMBER_PROPERTIES_CHANGED,
            path,
        ))?;

        devices.insert(path.clone());

        Ok(())
    }

    fn get_current_devices(&self, connection: &dbus::Connection) -> Result<Vec<dbus::Path<'_>>> {
        let message = dbus::Message::init_method_call(
            INTERFACE_UPOWER,
            PATH_UPOWER,
            INTERFACE_UPOWER,
            MEMBER_ENUMERATE_DEVICES,
        )?;

        let response = connection.send_message(message)?;

        response.return_value::<Vec<dbus::Path<'_>>>()
    }

    fn get_device_name<'a>(&self, path: &'a dbus::Path<'_>) -> Result<&'a str> {
        let string = path.as_cstr().to_str().wrap_error(
            FEATURE_NAME,
            "failed to create utf8 string of dbus object path",
        )?;

        Ok(string.trim_start_matches(PATH_BATTERY_DEVICES_PREFIX))
    }

    fn remove_device<'a>(
        &self,
        connection: &dbus::Connection,
        devices: &mut HashSet<dbus::Path<'a>>,
        path: &dbus::Path<'a>,
    ) -> Result<()> {
        if !devices.contains(path) {
            return Ok(());
        }

        connection.remove_match(dbus::Match::new(
            INTERFACE_DBUS_PROPERTIES,
            MEMBER_PROPERTIES_CHANGED,
            path,
        ))?;

        devices.remove(path);

        Ok(())
    }
}

impl thread::Runnable for DbusWatcher {
    fn run(&self) -> Result<()> {
        let connection = dbus::Connection::init()?;

        connection.add_match(dbus::Match::new(INTERFACE_UPOWER, None, PATH_UPOWER))?;

        let mut devices = HashSet::new();

        let current_devices = self.get_current_devices(&connection)?;

        for device in current_devices {
            self.add_device(&connection, &mut devices, &device)?;
        }

        // Manually send message before listen because `get_current_devices` waits for
        // dbus method call with a 2 seconds timeout. While waiting it's possible that
        // the initial `update` has already been triggered, so the status bar would show
        // the "no battery" information.
        communication::send_message(self.id, &self.sender)?;

        connection.listen_for_signals(|signal| {
            if signal.is_interface(INTERFACE_UPOWER)? {
                let path = signal.return_value::<dbus::Path<'_>>()?;

                if signal.is_member(MEMBER_DEVICE_ADDED)? {
                    self.add_device(&connection, &mut devices, &path)?;
                } else {
                    self.remove_device(&connection, &mut devices, &path)?;
                }

                communication::send_message(self.id, &self.sender)?;
            } else if signal.is_member(MEMBER_PROPERTIES_CHANGED)? {
                // wait for /sys/class/power_supply files updates
                thread::sleep_secs(2);

                communication::send_message(self.id, &self.sender)?;
            }

            Ok(())
        })
    }
}
