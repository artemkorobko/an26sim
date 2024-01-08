use std::time;

use crate::{driver::UsbDevice, error::DriverError};

pub enum Inbound {
    GetVersion,
    EnableGenerator(u8, u8, u16, u16),
    DisableGenerator(u8),
    StartProducer(u8),
    StopProducer,
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
            Inbound::EnableGenerator(index, period, value, step) => {
                let buf = [
                    2,
                    index,
                    period,
                    value as u8,
                    (value >> 8) as u8,
                    step as u8,
                    (step >> 8) as u8,
                ];
                self.write_all(&buf)
            }
            Inbound::DisableGenerator(index) => {
                let buf = [3, index];
                self.write_all(&buf)
            }
            Inbound::StartProducer(fps) => {
                let buf = [4, fps];
                self.write_all(&buf)
            }
            Inbound::StopProducer => {
                let buf = [5];
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

    #[test]
    fn start_producer() {
        let mut device = find_device();

        let size = device
            .write_ex(Inbound::EnableGenerator(0, 1, 100, 100))
            .expect("Error sending start request");
        assert_eq!(size, 7);
        let size = device
            .write_ex(Inbound::StartProducer(20))
            .expect("Error sending start producer request");
        assert_eq!(size, 2);
    }

    #[test]
    fn stop_producer() {
        let mut device = find_device();

        let size = device
            .write_ex(Inbound::StopProducer)
            .expect("Error sending stop producer request");

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
