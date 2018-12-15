use super::get_value;
use super::AcAdapter;
use super::BatteryDevice;
use super::DeviceMessage;
use super::FEATURE_NAME;
use error::*;
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Debug)]
pub(super) struct BatteryManager {
    ac_name: String,
    debug: bool,
    devices: HashMap<String, BatteryDevice>,
    rx_devices: mpsc::Receiver<DeviceMessage>,
}

impl BatteryManager {
    pub(super) fn new(debug: bool, rx_devices: mpsc::Receiver<DeviceMessage>) -> Result<Self> {
        Ok(Self {
            ac_name: AcAdapter::get_current()?,
            debug,
            devices: HashMap::new(),
            rx_devices,
        })
    }

    pub(super) fn get_devices(&mut self) -> &mut HashMap<String, BatteryDevice> {
        &mut self.devices
    }

    pub(super) fn is_ac_online(&self) -> Result<bool> {
        Ok(get_value(&self.ac_name, "online")? == 1)
    }

    pub(super) fn update_devices_list(&mut self) -> Result<()> {
        while let Ok(message) = self.rx_devices.try_recv() {
            match message {
                DeviceMessage::Added(name) => {
                    if self.debug {
                        println!("update {} add: {}", FEATURE_NAME, &name);
                    }
                    let device = BatteryDevice::new(&name)?;
                    self.devices.insert(name, device);
                },
                DeviceMessage::Removed(name) => {
                    if self.debug {
                        println!("update {} remove: {}", FEATURE_NAME, &name);
                    }
                    self.devices.remove(&name);
                },
            }
        }

        Ok(())
    }
}
