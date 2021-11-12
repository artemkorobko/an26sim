use std::time;

use crate::{device_lookup::DeviceLookup, endpoint_lookup::EndpointLookup, error::DriverError};

pub struct Device<T: rusb::UsbContext> {
    device: DeviceLookup<T>,
    readable_endpoint: EndpointLookup,
    writeable_endpoint: EndpointLookup,
}

impl<T: rusb::UsbContext> Drop for Device<T> {
    fn drop(&mut self) {
        self.readable_endpoint
            .close(&mut self.device.handle)
            .expect("Unable to close read endpoint");
        self.writeable_endpoint
            .close(&mut self.device.handle)
            .expect("Unable to close write endpoint");
    }
}

impl<T: rusb::UsbContext> Device<T> {
    pub fn from(
        mut device: DeviceLookup<T>,
        mut readable_endpoint: EndpointLookup,
        mut writeable_endpoint: EndpointLookup,
    ) -> Result<Self, DriverError> {
        readable_endpoint.open(&mut device.handle)?;
        writeable_endpoint.open(&mut device.handle)?;

        Ok(Self {
            device,
            readable_endpoint,
            writeable_endpoint,
        })
    }

    pub fn check(&mut self, timeout: time::Duration) -> Result<bool, DriverError> {
        self.write_ping(timeout)?;
        if self.read_pong(timeout)? {
            self.write_ping(timeout)?;
            self.read_pong(timeout)
        } else {
            Ok(false)
        }
    }

    pub fn write(&mut self, buf: &[u8], timeout: time::Duration) -> Result<usize, DriverError> {
        self.writeable_endpoint
            .write(&self.device.handle, buf, timeout)
    }

    pub fn write_all(
        &mut self,
        buf: &[u8],
        timeout: time::Duration,
        mut retries: usize,
    ) -> Result<bool, DriverError> {
        let bytes_total = buf.len();
        let mut bytes_written = self.write(buf, timeout)?;
        while bytes_written < bytes_total && retries > 0 {
            bytes_written = self.write(&buf[bytes_written..], timeout)?;
            retries -= 1;
        }
        Ok(retries > 0)
    }

    pub fn read(&mut self, buf: &mut [u8], timeout: time::Duration) -> Result<usize, DriverError> {
        self.readable_endpoint
            .read(&self.device.handle, buf, timeout)
    }

    pub fn read_all(
        &mut self,
        buf: &mut [u8],
        timeout: time::Duration,
        mut retries: usize,
    ) -> Result<bool, DriverError> {
        let bytes_total = buf.len();
        let mut bytes_read = self.read(buf, timeout)?;
        while bytes_read < bytes_total && retries > 0 {
            bytes_read = self.read(&mut buf[bytes_read..], timeout)?;
            retries -= 1;
        }
        Ok(retries > 0)
    }

    fn write_ping(&mut self, timeout: time::Duration) -> Result<bool, DriverError> {
        let buf = [1];
        let bytes_written = self.write(&buf, timeout)?;
        Ok(bytes_written == buf.len())
    }

    fn read_pong(&mut self, timeout: time::Duration) -> Result<bool, DriverError> {
        let mut buf = [0u8; 1];
        let bytes_read = self.read(&mut buf, timeout)?;
        Ok(bytes_read == 1 && buf[0] == 2)
    }
}
