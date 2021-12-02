use std::time;

use crate::{driver::UsbDevice, error::DriverError};

pub enum UsbInPacket {
    GetVersion,
    Ping(u8, u8),
    LedOn,
    LedOff,
    SetParam(u8, u16),
    GetParam(u8),
}

#[derive(Debug, PartialEq)]
pub enum UsbOutPacket {
    Error(u8),
    Version(u8, u8),
    Pong(u8, u8),
    Param(u8, u16),
    Unknown,
}

pub trait SM2MDevice {
    fn write_packet(&mut self, packet: UsbInPacket) -> Result<usize, DriverError>;
    fn read_packet(&mut self) -> Result<UsbOutPacket, DriverError>;
}

impl SM2MDevice for UsbDevice {
    fn write_packet(&mut self, packet: UsbInPacket) -> Result<usize, DriverError> {
        match packet {
            UsbInPacket::GetVersion => {
                let buf = [1];
                self.write_all(&buf)
            }
            UsbInPacket::Ping(version, payload) => {
                let buf = [2 | payload << 4, version];
                self.write_all(&buf)
            }
            UsbInPacket::LedOn => self.write_all(&[3 | 1 << 4]),
            UsbInPacket::LedOff => self.write_all(&[3]),
            UsbInPacket::SetParam(index, value) => {
                let buf = [4 | index << 4, value as u8, (value >> 8) as u8];
                self.write_all(&buf)
            }
            UsbInPacket::GetParam(index) => {
                let buf = [5 | index << 4];
                self.write_all(&buf)
            }
        }
    }

    fn read_packet(&mut self) -> Result<UsbOutPacket, DriverError> {
        const READ_TIMEOUT: time::Duration = time::Duration::from_secs(1);
        let mut buf = [0u8; 64];
        self.read(&mut buf, READ_TIMEOUT)?;
        let opcode = buf[0] & 0x0f;
        let packet = match opcode {
            1 => {
                let reason = buf[0] >> 4;
                UsbOutPacket::Error(reason)
            }
            2 => {
                let major = buf[0] >> 4;
                let minor = buf[1];
                UsbOutPacket::Version(major, minor)
            }
            3 => {
                let payload = buf[0] >> 4;
                let version = buf[1];
                UsbOutPacket::Pong(version, payload)
            }
            4 => {
                let index = buf[0] >> 4;
                let param = buf[1] as u16 | (buf[2] as u16) << 8;
                UsbOutPacket::Param(index, param)
            }
            _ => UsbOutPacket::Unknown,
        };
        Ok(packet)
    }
}
