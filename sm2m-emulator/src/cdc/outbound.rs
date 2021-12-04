use usb_device::UsbError;

use super::device::CdcDevice;

pub enum UsbOutbound {
    Version(u8, u8, u8),
}

pub trait Writer {
    fn write_ex(&mut self, packet: UsbOutbound) -> Result<usize, UsbError>;
}

impl Writer for CdcDevice {
    fn write_ex(&mut self, packet: UsbOutbound) -> Result<usize, UsbError> {
        match packet {
            UsbOutbound::Version(major, minor, patch) => {
                let buf = [1, major, minor, patch];
                self.write_all(&buf)
            }
        }
    }
}
