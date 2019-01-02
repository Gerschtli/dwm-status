use super::get_value;
use super::AcAdapter;
use super::BatteryDevice;
use super::DeviceMessage;
use error::*;
use std::collections::HashMap;
use wrapper::channel;

pub(super) struct BatteryManager {
    ac_name: &'static str,
    devices: HashMap<String, BatteryDevice>,
    receiver_devices: channel::Receiver<DeviceMessage>,
}

impl BatteryManager {
    pub(super) fn init(receiver_devices: channel::Receiver<DeviceMessage>) -> Result<Self> {
        Ok(Self {
            ac_name: AcAdapter::get_current()?,
            devices: HashMap::new(),
            receiver_devices,
        })
    }

    pub(super) fn get_devices(&mut self) -> &mut HashMap<String, BatteryDevice> {
        &mut self.devices
    }

    pub(super) fn is_ac_online(&self) -> Result<bool> {
        Ok(get_value(self.ac_name, "online")? == 1)
    }

    pub(super) fn update_devices_list(&mut self) -> Result<()> {
        while let Ok(message) = self.receiver_devices.read() {
            match message {
                DeviceMessage::Added(name) => {
                    info!("Detected connected battery: adding {}", &name);
                    let device = BatteryDevice::new(&name[..]);
                    self.devices.insert(name, device);
                },
                DeviceMessage::Removed(name) => {
                    info!("Detected disconnected battery: removing {}", &name);
                    self.devices.remove(&name);
                },
            }
        }

        Ok(())
    }
}
