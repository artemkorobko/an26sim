use std::time;

use crate::{driver::UsbDevice, error::DriverError};

pub enum Inbound {
    GetVersion,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outbound {
    Version(u8, u8, u8),
    Unknown,
}

pub trait DecoderDevice {
    fn write_ex(&mut self, packet: Inbound) -> Result<usize, DriverError>;
    fn read_ex(&mut self) -> Result<Outbound, DriverError>;
}

impl DecoderDevice for UsbDevice {
    fn write_ex(&mut self, packet: Inbound) -> Result<usize, DriverError> {
        match packet {
            Inbound::GetVersion => {
                let buf = [1];
                self.write_all(&buf)
            }
        }
    }

    fn read_ex(&mut self) -> Result<Outbound, DriverError> {
        const READ_TIMEOUT: time::Duration = time::Duration::from_secs(1);
        let mut buf = [0u8; 64];
        self.read(&mut buf, READ_TIMEOUT)?;
        let opcode = buf[0] & 0x0f;
        let packet = match opcode {
            1 => {
                let major = buf[1];
                let minor = buf[2];
                let patch = buf[3];
                Outbound::Version(major, minor, patch)
            }
            _ => Outbound::Unknown,
        };
        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use crate::driver::UsbDriver;

    use super::*;

    const IO_TIMEOUT: time::Duration = time::Duration::from_secs(1);

    #[test]
    fn get_version() {
        let mut device = find_device();

        let size = device
            .write_ex(Inbound::GetVersion)
            .expect("Error sending version request");
        assert_eq!(size, 1);
        let packet = device.read_ex().expect("Error reading packet from device");
        assert_ne!(packet, Outbound::Unknown);

        println!("{:?}", packet);
    }

    fn find_device() -> UsbDevice {
        let mut driver = UsbDriver::new().expect("Error initializing driver");
        let mut device = driver
            .find_decoder(IO_TIMEOUT)
            .expect("Error finding emulator")
            .expect("No emulator found");
        device.reset().expect("Error resetting device");
        device
    }
}
