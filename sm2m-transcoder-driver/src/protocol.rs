use std::time;

use crate::{driver::USBDevice, error::DriverError};

pub enum Request {
    Ping(u8, u8),
    LedOn,
    LedOff,
    SetParam(u8, u16),
    GetParam(u8),
}

#[derive(Debug, PartialEq)]
pub enum Response {
    Pong(u8, u8),
    Param(u8, u16),
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
                let buf = [1u8 | payload << 4, version];
                self.write_all(&buf)
            }
            Request::LedOn => self.write_all(&18u8.to_le_bytes()),
            Request::LedOff => self.write_all(&2u8.to_le_bytes()),
            Request::SetParam(index, value) => {
                let buf = [3u8 | index << 4, value as u8, (value >> 8) as u8];
                self.write_all(&buf)
            }
            Request::GetParam(index) => {
                let buf = [4u8 | index << 4];
                self.write_all(&buf)
            }
        }
    }

    fn read_response(&mut self) -> Result<Response, DriverError> {
        const READ_TIMEOUT: time::Duration = time::Duration::from_secs(1);
        let mut buf = [0u8; 64];
        self.read(&mut buf, READ_TIMEOUT)?;
        let opcode = buf[0] & 0x0f;
        let response = match opcode {
            1 => {
                let payload = buf[0] >> 4;
                let version = buf[1];
                Response::Pong(version, payload)
            }
            4 => {
                let index = buf[0] >> 4;
                let param = buf[1] as u16 | (buf[2] as u16) << 8;
                Response::Param(index, param)
            }
            _ => Response::Unknown,
        };
        Ok(response)
    }
}
