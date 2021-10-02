use std::time;

use crate::{device_lookup::DeviceLookup, endpoint_lookup::EndpointLookup, error::DriverError};

pub struct Device<T: rusb::UsbContext> {
    device: DeviceLookup<T>,
    readable_endpoint: EndpointLookup,
    writeable_endpoint: EndpointLookup,
}

impl<T: rusb::UsbContext> Drop for Device<T> {
    fn drop(&mut self) {
        log::debug!("Close readable endpoint");
        if let Err(error) = self.readable_endpoint.close(self.device.handle()) {
            log::error!("{:?}", error);
        }
        log::debug!("Close writeable endpoint");
        if let Err(error) = self.writeable_endpoint.close(self.device.handle()) {
            log::error!("{:?}", error);
        }
    }
}

impl<T: rusb::UsbContext> Device<T> {
    pub fn from(
        mut device: DeviceLookup<T>,
        mut readable_endpoint: EndpointLookup,
        mut writeable_endpoint: EndpointLookup,
    ) -> Result<Self, DriverError> {
        log::debug!("Open readable endpoint");
        readable_endpoint.open(&mut device.handle())?;
        log::debug!("Open writeable endpoint");
        writeable_endpoint.open(&mut device.handle())?;

        Ok(Self {
            device,
            readable_endpoint,
            writeable_endpoint,
        })
    }

    pub fn check(&mut self, timeout: time::Duration) -> Result<bool, DriverError> {
        log::debug!("Send PING");
        self.write_ping(timeout)?;
        log::debug!("Read PONG");
        let result = self.read_pong(timeout)?;
        if result != 2 {
            log::debug!("Invalid PONG response: {}", result);
            Ok(false)
        } else {
            log::debug!("Send PING");
            self.write_ping(timeout)?;
            log::debug!("Read PONG");
            let result = self.read_pong(timeout)? == 2;
            Ok(result)
        }
    }

    fn write_ping(&mut self, timeout: time::Duration) -> Result<usize, DriverError> {
        let buf = [1];
        self.writeable_endpoint
            .write(self.device.handle(), &buf, timeout)
    }

    fn read_pong(&mut self, timeout: time::Duration) -> Result<u8, DriverError> {
        let mut buf = [0u8; 10];
        let size = self
            .readable_endpoint
            .read(self.device.handle(), &mut buf, timeout)?;
        if size == 1 {
            Ok(buf[0])
        } else {
            Ok(0)
        }
    }
}
