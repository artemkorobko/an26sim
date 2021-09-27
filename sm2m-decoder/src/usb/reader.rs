use super::{cdc_device::CDCDevice, command::USBCommand};

#[derive(Default)]
pub struct USBReader {}

impl USBReader {
    pub fn read_command(&mut self, device: &mut CDCDevice) -> Option<USBCommand> {
        let mut data = [0u8; 1];
        if let Ok(size) = device.read(&mut data) {
            if size > 0 {
                return Some(data[0].into());
            }
        }

        None
    }
}
