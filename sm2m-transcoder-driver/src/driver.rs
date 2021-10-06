use std::time;

use crate::{
    device::Device, device_lookup::DeviceLookup, device_lookup_helper, error::DriverError,
};

pub struct Driver {
    context: rusb::Context,
}

impl Driver {
    pub fn new() -> Result<Self, DriverError> {
        let context = rusb::Context::new().map_err(DriverError::Init)?;
        Ok(Self { context })
    }

    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_owned()
    }

    pub fn libusb_version() -> String {
        let version = rusb::version();
        format!(
            "{}.{}.{}.{}",
            version.major(),
            version.minor(),
            version.micro(),
            version.nano()
        )
    }

    pub fn find_encoder(
        &mut self,
        timeout: time::Duration,
    ) -> Result<Option<Device<rusb::Context>>, DriverError> {
        log::debug!("Search for supported encoder");
        let device_lookup = device_lookup_helper::find_encoder(&mut self.context, timeout)?;
        match device_lookup {
            Some(device_lookup) => Ok(Some(Self::create_device(device_lookup)?)),
            None => Ok(None),
        }
    }

    pub fn find_decoder(
        &mut self,
        timeout: time::Duration,
    ) -> Result<Option<Device<rusb::Context>>, DriverError> {
        log::debug!("Search for supported decoder");
        let device_lookup = device_lookup_helper::find_decoder(&mut self.context, timeout)?;
        match device_lookup {
            Some(device_lookup) => Ok(Some(Self::create_device(device_lookup)?)),
            None => Ok(None),
        }
    }

    fn create_device<T: rusb::UsbContext>(
        mut device_lookup: DeviceLookup<T>,
    ) -> Result<Device<T>, DriverError> {
        log::debug!("Search for available readable endpoints");
        let readable_endpoint = device_lookup.find_readable_endpoint()?;
        log::debug!("Search for available writeable endpoints");
        let writeable_endpoint = device_lookup.find_writeable_endpoint()?;
        Device::from(device_lookup, readable_endpoint, writeable_endpoint)
    }
}
