use super::FEATURE_NAME;
use async;
use dbus;
use error::*;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use std::time;

const INTERFACE_DBUS_PROPERTIES: &str = "org.freedesktop.DBus.Properties";
const INTERFACE_UPOWER: &str = "org.freedesktop.UPower";
const MEMBER_DEVICE_ADDED: &str = "DeviceAdded";
const MEMBER_ENUMERATE_DEVICES: &str = "EnumerateDevices";
const MEMBER_PROPERTIES_CHANGED: &str = "PropertiesChanged";
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
        let connection = dbus::Connection::get_private(dbus::BusType::System)
            .wrap_error(FEATURE_NAME, "failed to connect to dbus")?;

        Ok(DbusWatcher {
            connection,
            id,
            tx,
            tx_devices,
        })
    }

    pub fn start(&self) -> Result<()> {
        self.connection
            .add_match(&format!(
                "type='signal',path='{}',interface='{}'",
                PATH_UPOWER, INTERFACE_UPOWER
            )).wrap_error(FEATURE_NAME, "failed to add match")?;

        let mut devices = HashSet::new();

        for device in self.get_current_devices()? {
            self.add_device(&mut devices, &device)?;
        }

        for item in self.connection.iter(300_000) {
            if let dbus::ConnectionItem::Signal(signal) = item {
                if self.is_upower_interface(&signal.interface())? {
                    let path = signal
                        .read1::<dbus::Path>()
                        .wrap_error(FEATURE_NAME, "failed to read path of dbus signal")?;

                    if self.is_device_added_member(&signal.member())? {
                        self.add_device(&mut devices, &path)?;
                    } else {
                        self.remove_device(&mut devices, &path)?;
                    }
                } else if self.is_properties_changed_member(&signal.member())? {
                    // wait for /sys/class/power_supply files updates
                    thread::sleep(time::Duration::from_secs(2));
                }

                async::send_message(FEATURE_NAME, &self.id, &self.tx);
            }
        }

        Ok(())
    }

    fn add_device<'a>(
        &self,
        devices: &mut HashSet<dbus::Path<'a>>,
        path: &dbus::Path<'a>,
    ) -> Result<()> {
        let name = self.get_device_name(path)?;

        // ignore line power devices
        if name.starts_with(PATH_DEVICES_PREFIX) || devices.contains(path) {
            return Ok(());
        }

        self.connection
            .add_match(&self.build_properties_changed_match(path))
            .wrap_error(FEATURE_NAME, "failed to add device match")?;

        self.tx_devices
            .send(DeviceMessage::Added(String::from(name)))
            .wrap_error(FEATURE_NAME, "failed to send device added message")?;

        devices.insert(path.clone());

        Ok(())
    }

    fn build_properties_changed_match(&self, path: &dbus::Path) -> String {
        format!(
            "type='signal',path='{}',interface='{}',member='{}'",
            path, INTERFACE_DBUS_PROPERTIES, MEMBER_PROPERTIES_CHANGED
        )
    }

    fn is_device_added_member(&self, member: &Option<dbus::Member>) -> Result<bool> {
        let added_member = dbus::Member::new(MEMBER_DEVICE_ADDED)
            .wrap_error(FEATURE_NAME, "failed to create member instance")?;

        Ok(self.is_some_equal(member, &added_member))
    }

    fn is_properties_changed_member(&self, member: &Option<dbus::Member>) -> Result<bool> {
        let added_member = dbus::Member::new(MEMBER_PROPERTIES_CHANGED)
            .wrap_error(FEATURE_NAME, "failed to create member instance")?;

        Ok(self.is_some_equal(member, &added_member))
    }

    fn is_some_equal<T: PartialEq>(&self, instance: &Option<T>, compare: &T) -> bool {
        match *instance {
            Some(ref instance) if instance == compare => true,
            _ => false,
        }
    }

    fn is_upower_interface(&self, interface: &Option<dbus::Interface>) -> Result<bool> {
        let upower_interface = dbus::Interface::new(INTERFACE_UPOWER)
            .wrap_error(FEATURE_NAME, "failed to create interface instance")?;

        Ok(self.is_some_equal(interface, &upower_interface))
    }

    fn get_current_devices(&self) -> Result<Vec<dbus::Path>> {
        let message = dbus::Message::new_method_call(
            INTERFACE_UPOWER,
            PATH_UPOWER,
            INTERFACE_UPOWER,
            MEMBER_ENUMERATE_DEVICES,
        ).wrap_error(
            FEATURE_NAME,
            "failed to create dbus message enumerate devices",
        )?;

        let response = self.connection
            .send_with_reply_and_block(message, 2000) // 2 seconds timeout
            .wrap_error(FEATURE_NAME, "failed to call dbus enumerate devices")?;

        response.read1::<Vec<dbus::Path>>().wrap_error(
            FEATURE_NAME,
            "failed to read dbus response enumerate devices",
        )
    }

    fn get_device_name<'a>(&self, path: &'a dbus::Path) -> Result<&'a str> {
        let string = path.as_cstr().to_str().wrap_error(
            FEATURE_NAME,
            "failed to create utf8 string of dbus object path",
        )?;

        Ok(string.trim_left_matches(PATH_BATTERY_DEVICES_PREFIX))
    }

    fn remove_device<'a>(
        &self,
        devices: &mut HashSet<dbus::Path<'a>>,
        path: &dbus::Path<'a>,
    ) -> Result<()> {
        if !devices.contains(path) {
            return Ok(());
        }

        let name = self.get_device_name(path)?;

        self.connection
            .remove_match(&self.build_properties_changed_match(path))
            .wrap_error(FEATURE_NAME, "failed to remove device match")?;

        self.tx_devices
            .send(DeviceMessage::Removed(String::from(name)))
            .wrap_error(FEATURE_NAME, "failed to send device removed message")?;

        devices.remove(path);

        Ok(())
    }
}
