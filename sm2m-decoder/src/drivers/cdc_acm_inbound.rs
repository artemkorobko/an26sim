use usb_device::UsbError;

use super::cdc_acm::Device;

pub enum Inbound {
    FirmwareVersion,
    Unknown,
}

pub trait Reader {
    fn read_inbound(&mut self) -> Result<Inbound, UsbError>;
}

impl Reader for Device {
    fn read_inbound(&mut self) -> Result<Inbound, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let opcode = buf[0];
        let packet = match opcode {
            1 => Inbound::FirmwareVersion,
            _ => Inbound::Unknown,
        };
        Ok(packet)
    }
}
