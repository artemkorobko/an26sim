use usb_device::UsbError;

use super::cdc_acm::Device;

pub enum Outbound {
    Version(u8, u8, u8),
}

pub trait Writer {
    fn write_outbound(&mut self, packet: Outbound) -> Result<usize, UsbError>;
}

impl Writer for Device {
    fn write_outbound(&mut self, packet: Outbound) -> Result<usize, UsbError> {
        match packet {
            Outbound::Version(major, minor, patch) => {
                let buf = [1, major, minor, patch];
                self.write_all(&buf)
            }
        }
    }
}
