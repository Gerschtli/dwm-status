use super::FEATURE_NAME;
use async;
use dbus;
use dbus::data;
use dbus_lib;
use error::*;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use std::time;

const PATH_BATTERY_DEVICES_PREFIX: &str = "/org/freedesktop/UPower/devices/battery_";
const PATH_DEVICES_PREFIX: &str = "/org/freedesktop/UPower/devices";
const PATH_UPOWER: &str = "/org/freedesktop/UPower";

pub enum DeviceMessage {
    Added(String),
    Removed(String),
}

pub struct DbusWatcher {
    connection: dbus::Connection,
    id: String,
    tx: mpsc::Sender<async::Message>,
    tx_devices: mpsc::Sender<DeviceMessage>,
}

impl DbusWatcher {
    pub fn new(
        id: String,
        tx: mpsc::Sender<async::Message>,
        tx_devices: mpsc::Sender<DeviceMessage>,
    ) -> Result<Self> {
        Ok(DbusWatcher {
            connection: dbus::Connection::new()?,
            id,
            tx,
            tx_devices,
        })
    }

    pub fn start(&self) -> Result<()> {
        self.connection.add_match(data::Match {
            interface: data::Interface::UPOWER,
            member: None,
            path: PATH_UPOWER,
        })?;

        let mut devices = HashSet::new();

        for device in self.get_current_devices()? {
            self.add_device(&mut devices, &device)?;
        }

        self.connection.listen_for_signals(|signal| {
            if signal.is_interface(data::Interface::UPOWER)? {
                let path = signal.return_value::<dbus_lib::Path>()?;

                if signal.is_member(data::Member::DEVICE_ADDED)? {
                    self.add_device(&mut devices, &path)?;
                } else {
                    self.remove_device(&mut devices, &path)?;
                }
            } else if signal.is_member(data::Member::PROPERTIES_CHANGED)? {
                // wait for /sys/class/power_supply files updates
                thread::sleep(time::Duration::from_secs(2));
            }

            async::send_message(FEATURE_NAME, &self.id, &self.tx);
            Ok(())
        })
    }

    fn add_device<'a>(
        &self,
        devices: &mut HashSet<dbus_lib::Path<'a>>,
        path: &dbus_lib::Path<'a>,
    ) -> Result<()> {
        let name = self.get_device_name(path)?;

        // ignore line power devices
        if name.starts_with(PATH_DEVICES_PREFIX) || devices.contains(path) {
            return Ok(());
        }

        self.connection.add_match(data::Match {
            interface: data::Interface::DBUS_PROPERTIES,
            member: Some(data::Member::PROPERTIES_CHANGED),
            path,
        })?;

        self.tx_devices
            .send(DeviceMessage::Added(String::from(name)))
            .wrap_error(FEATURE_NAME, "failed to send device added message")?;

        devices.insert(path.clone());

        Ok(())
    }

    fn get_current_devices(&self) -> Result<Vec<dbus_lib::Path>> {
        let message = dbus::Message::new_method_call(
            &data::Interface::UPOWER,
            PATH_UPOWER,
            &data::Interface::UPOWER,
            &data::Member::ENUMERATE_DEVICES,
        )?;

        let response = self.connection.send_message(message)?;

        response.return_value::<Vec<dbus_lib::Path>>()
    }

    fn get_device_name<'a>(&self, path: &'a dbus_lib::Path) -> Result<&'a str> {
        let string = path.as_cstr().to_str().wrap_error(
            FEATURE_NAME,
            "failed to create utf8 string of dbus object path",
        )?;

        Ok(string.trim_left_matches(PATH_BATTERY_DEVICES_PREFIX))
    }

    fn remove_device<'a>(
        &self,
        devices: &mut HashSet<dbus_lib::Path<'a>>,
        path: &dbus_lib::Path<'a>,
    ) -> Result<()> {
        if !devices.contains(path) {
            return Ok(());
        }

        let name = self.get_device_name(path)?;

        self.connection.remove_match(data::Match {
            interface: data::Interface::DBUS_PROPERTIES,
            member: Some(data::Member::PROPERTIES_CHANGED),
            path,
        })?;

        self.tx_devices
            .send(DeviceMessage::Removed(String::from(name)))
            .wrap_error(FEATURE_NAME, "failed to send device removed message")?;

        devices.remove(path);

        Ok(())
    }
}
