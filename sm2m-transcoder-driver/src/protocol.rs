use std::time;

use crate::{driver::USBDevice, error::DriverError};

pub enum Request {
    Ping(u8, u8),
    LedOn,
    LedOff,
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Pong(u8, u8),
    Unknown,
}

pub trait SM2MDevice {
    fn write_request(&mut self, request: Request) -> Result<usize, DriverError>;
    fn read_response(&mut self) -> Result<Response, DriverError>;
}

impl SM2MDevice for USBDevice {
    fn write_request(&mut self, request: Request) -> Result<usize, DriverError> {
        match request {
            Request::Ping(version, payload) => {
                let mut msg = 0x1u16;
                msg |= (version as u16) << 4;
                msg |= (payload as u16) << 12;
                self.write_all(&msg.to_le_bytes())
            }
            Request::LedOn => self.write_all(&18u8.to_le_bytes()),
            Request::LedOff => self.write_all(&2u8.to_le_bytes()),
        }
    }

    fn read_response(&mut self) -> Result<Response, DriverError> {
        const READ_TIMEOUT: time::Duration = time::Duration::from_secs(1);
        let mut buf = [0u8; 64];
        self.read(&mut buf, READ_TIMEOUT)?;
        let opcode = buf[0] & 0x0f;
        let response = match opcode {
            1 => {
                let version = buf[1] << 4 | buf[0] >> 4;
                let payload = buf[1] >> 4 & 0xf;
                Response::Pong(version, payload)
            }
            _ => Response::Unknown,
        };
        Ok(response)
    }
}
