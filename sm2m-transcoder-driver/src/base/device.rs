use std::time;

use crate::error::DriverError;

use super::{device_lookup::DeviceLookup, endpoint_lookup::EndpointLookup};

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

    pub fn reset(&mut self) -> Result<(), DriverError> {
        self.device.handle.reset().map_err(|error| {
            DriverError::Reset(
                error,
                self.device.descriptor.vendor_id(),
                self.device.descriptor.product_id(),
            )
        })
    }

    pub fn write(&mut self, buf: &[u8], timeout: time::Duration) -> Result<usize, DriverError> {
        self.writeable_endpoint
            .write(&self.device.handle, buf, timeout)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<usize, DriverError> {
        self.try_write_all(buf, time::Duration::MAX, usize::MAX)
    }

    pub fn try_write_all(
        &mut self,
        buf: &[u8],
        timeout: time::Duration,
        mut retries: usize,
    ) -> Result<usize, DriverError> {
        let bytes_total = buf.len();
        let mut bytes_written = self.write(buf, timeout)?;
        while bytes_written < bytes_total && retries > 0 {
            bytes_written += self.write(&buf[bytes_written..], timeout)?;
            retries -= 1;
        }
        Ok(bytes_written)
    }

    pub fn read(&mut self, buf: &mut [u8], timeout: time::Duration) -> Result<usize, DriverError> {
        self.readable_endpoint
            .read(&self.device.handle, buf, timeout)
    }

    pub fn read_all(&mut self, buf: &mut [u8]) -> Result<usize, DriverError> {
        self.try_read_all(buf, time::Duration::MAX, usize::MAX)
    }

    pub fn try_read_all(
        &mut self,
        buf: &mut [u8],
        timeout: time::Duration,
        mut retries: usize,
    ) -> Result<usize, DriverError> {
        let bytes_total = buf.len();
        let mut bytes_read = self.read(buf, timeout)?;
        while bytes_read < bytes_total && retries > 0 {
            println!("123");
            bytes_read += self.read(&mut buf[bytes_read..], timeout)?;
            retries -= 1;
        }
        Ok(bytes_read)
    }
}
