use std::time;

use crate::{driver::UsbDevice, error::DriverError};

pub enum Inbound {
    GetVersion,
    UpdateParam(u8, u16),
    EnableGenerator(u8, u8, u16),
    DisableGenerator(u8),
    Start(u8, u16),
    Stop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outbound {
    Version(u8, u8, u8),
    Params(u16, [u16; 12]),
    Unknown,
}

pub trait EmulatorDevice {
    fn write_ex(&mut self, packet: Inbound) -> Result<usize, DriverError>;
    fn read_ex(&mut self) -> Result<Outbound, DriverError>;
}

impl EmulatorDevice for UsbDevice {
    fn write_ex(&mut self, packet: Inbound) -> Result<usize, DriverError> {
        match packet {
            Inbound::GetVersion => {
                let buf = [1];
                self.write_all(&buf)
            }
            Inbound::UpdateParam(_, _) => todo!(),
            Inbound::EnableGenerator(_, _, _) => todo!(),
            Inbound::DisableGenerator(_) => todo!(),
            Inbound::Start(fps, marker) => {
                let buf = [5, fps, marker as u8, (marker >> 8) as u8];
                self.write_all(&buf)
            }
            Inbound::Stop => {
                let buf = [6];
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
        let packet = device
            .read_ex()
            .expect("Error reading packet from device");
        assert_ne!(packet, Outbound::Unknown);

        println!("{:?}", packet);
    }

    #[test]
    fn start() {
        let mut device = find_device();

        let size = device
            .write_ex(Inbound::Start(20, 0))
            .expect("Error sending start request");

        assert_eq!(size, 4);
    }

    #[test]
    fn stop() {
        let mut device = find_device();

        let size = device
            .write_ex(Inbound::Stop)
            .expect("Error sending stop request");

        assert_eq!(size, 1);
    }

    fn find_device() -> UsbDevice {
        let mut driver = UsbDriver::new().expect("Error initializing driver");
        let mut device = driver
            .find_emulator(IO_TIMEOUT)
            .expect("Error finding emulator")
            .expect("No emulator found");
        device.reset().expect("Error resetting device");
        device
    }
}
