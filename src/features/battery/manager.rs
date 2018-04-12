use super::get_value;
use super::AcAdapter;
use super::BatteryDevice;
use super::DeviceMessage;
use super::FEATURE_NAME;
use error::*;
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Debug)]
pub struct BatteryManager {
    ac_name: String,
    devices: HashMap<String, BatteryDevice>,
    rx_devices: mpsc::Receiver<DeviceMessage>,
}

impl BatteryManager {
    pub fn new(rx_devices: mpsc::Receiver<DeviceMessage>) -> Result<Self> {
        Ok(BatteryManager {
            ac_name: AcAdapter::get_current()?,
            devices: HashMap::new(),
            rx_devices,
        })
    }

    pub fn get_devices(&mut self) -> &mut HashMap<String, BatteryDevice> {
        &mut self.devices
    }

    pub fn is_ac_online(&self) -> Result<bool> {
        Ok(get_value(&self.ac_name, "online")? == 1)
    }

    pub fn update_devices_list(&mut self) -> Result<()> {
        while let Ok(message) = self.rx_devices.try_recv() {
            match message {
                DeviceMessage::Added(name) => {
                    println!("update {} add: {}", FEATURE_NAME, &name);
                    let device = BatteryDevice::new(&name)?;
                    self.devices.insert(name, device);
                },
                DeviceMessage::Removed(name) => {
                    println!("update {} remove: {}", FEATURE_NAME, &name);
                    self.devices.remove(&name);
                },
            }
        }

        Ok(())
    }
}
