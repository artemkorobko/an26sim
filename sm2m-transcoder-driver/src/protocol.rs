use std::time;

use rand::Rng;

use crate::{driver::USBDevice, error::DriverError};

pub trait SM2MDevice {
    fn write_ping(&mut self, version: u8) -> Result<u32, DriverError>;
    fn read_pong(&mut self, payload: u32) -> Result<bool, DriverError>;
    fn led_on(&mut self) -> Result<bool, DriverError>;
    fn led_off(&mut self) -> Result<bool, DriverError>;
}

const IO_TIMEOUT: time::Duration = time::Duration::from_secs(1);

impl SM2MDevice for USBDevice {
    fn write_ping(&mut self, version: u8) -> Result<u32, DriverError> {
        let mut request = rand::thread_rng().gen_range(0x10000u32..0xFFFF0000u32);
        request = (request & 0xffffff00) | 0x1;
        request = (request & 0xffff00ff) | ((version as u32) << 8);
        self.write(&request.to_le_bytes(), IO_TIMEOUT)?;
        Ok(request)
    }

    fn read_pong(&mut self, mut payload: u32) -> Result<bool, DriverError> {
        let mut response = [0u8; 4];
        self.read(&mut response, IO_TIMEOUT)?;
        let version = (payload >> 8) & 0xff;
        payload = (payload & 0xffff00ff) | (version + 1) << 8;
        Ok(u32::from_le_bytes(response) == payload)
    }

    fn led_on(&mut self) -> Result<bool, DriverError> {
        let request = [2u8, 1u8, 0u8, 0u8];
        let size = self.write(&request, IO_TIMEOUT)?;
        Ok(size == request.len())
    }

    fn led_off(&mut self) -> Result<bool, DriverError> {
        let request = [2u8, 0u8, 0u8, 0u8];
        let size = self.write(&request, IO_TIMEOUT)?;
        Ok(size == request.len())
    }
}
