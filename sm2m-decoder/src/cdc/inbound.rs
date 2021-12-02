use usb_device::UsbError;

use super::device::CdcDevice;

pub enum UsbInbound {
    GetVersion,
}

pub trait Reader {
    fn read_ex(&mut self) -> Result<Option<UsbInbound>, UsbError>;
}

impl Reader for CdcDevice {
    fn read_ex(&mut self) -> Result<Option<UsbInbound>, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let opcode = buf[0];
        let packet = match opcode {
            1 => Some(UsbInbound::GetVersion),
            _ => None,
        };
        Ok(packet)
    }
}
