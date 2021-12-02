use usb_device::UsbError;

use crate::generators::MAX_PARAMS_COUNT;

use super::device::CdcDevice;

pub enum UsbOutbound<'a> {
    Version(u8, u8, u8),
    Params(u16, &'a [u16; MAX_PARAMS_COUNT]),
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
            UsbOutbound::Params(marker, params) => {
                let mut buf = [0u8; (MAX_PARAMS_COUNT * 2) + 2 + 1];
                buf[0] = 2;
                buf[1] = marker as u8;
                buf[2] = (marker >> 8) as u8;
                let mut buf_idx = 3;
                for param in params {
                    buf[buf_idx] = *param as u8;
                    buf[buf_idx + 1] = (*param >> 8) as u8;
                    buf_idx += 2;
                }
                self.write_all(&buf)
            }
        }
    }
}
