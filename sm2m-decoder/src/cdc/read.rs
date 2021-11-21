use usb_device::UsbError;

use super::device::CdcDevice;

pub type RequestType = u32;

pub enum Request {
    Ping(u8, u16),
    LedOn,
    LedOff,
    Unknown,
}

pub trait RequestTypeEx {
    fn parse(&self) -> Request;
}

impl RequestTypeEx for RequestType {
    fn parse(&self) -> Request {
        let opcode = *self as u8;
        match opcode {
            1 => {
                let version = (self >> 8) as u8;
                let payload = (self >> 16) as u16;
                Request::Ping(version, payload)
            }
            2 => {
                let state = (self >> 8) as u8;
                if state == 0 {
                    Request::LedOff
                } else if state == 1 {
                    Request::LedOn
                } else {
                    Request::Unknown
                }
            }
            _ => Request::Unknown,
        }
    }
}

#[derive(Default)]
pub struct CdcReader {
    payload: RequestType,
    bytes_read: usize,
}

impl CdcReader {
    pub fn read_from(&mut self, device: &mut CdcDevice) -> Result<Option<RequestType>, UsbError> {
        let mut buffer = self.payload.to_le_bytes();
        let bytes_read = device.read(&mut buffer[self.bytes_read..])?;
        self.payload = RequestType::from_le_bytes(buffer);
        if bytes_read < buffer.len() {
            self.bytes_read += bytes_read;
            Ok(None)
        } else {
            self.bytes_read = 0;
            Ok(Some(self.payload))
        }
    }
}
