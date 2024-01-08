use usb_device::UsbError;

use super::device::CdcDevice;

pub enum UsbOutPacket {
    Version(u8, u8),
    Pong(u8, u8),
    Param(u8, u16),
}

pub trait PacketWriter {
    fn write_packet(&mut self, packet: UsbOutPacket) -> Result<usize, UsbError>;
}

impl PacketWriter for CdcDevice {
    fn write_packet(&mut self, packet: UsbOutPacket) -> Result<usize, UsbError> {
        match packet {
            UsbOutPacket::Version(major, minor) => {
                let buf = [2 | major << 4, minor];
                write_all(self, &buf)
            }
            UsbOutPacket::Pong(version, payload) => {
                let buf = [3 | payload << 4, version];
                write_all(self, &buf)
            }
            UsbOutPacket::Param(index, param) => {
                let buf = [4 | index << 4, param as u8, (param >> 8) as u8];
                write_all(self, &buf)
            }
        }
    }
}

fn write_all(device: &mut CdcDevice, buf: &[u8]) -> Result<usize, UsbError> {
    let mut sent = 0;
    while sent < buf.len() {
        sent += device.write(buf)?;
    }
    Ok(sent)
}
