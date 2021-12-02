use std::time;

use crate::{
    base::{device_lookup::DeviceLookup, device_lookup_helper},
    error::DriverError,
};

pub struct UsbDriver {
    context: rusb::Context,
}

pub type UsbDevice = crate::base::device::Device<rusb::Context>;

impl UsbDriver {
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

    pub fn find_emulator(
        &mut self,
        timeout: time::Duration,
    ) -> Result<Option<UsbDevice>, DriverError> {
        let device_lookup =
            device_lookup_helper::find_device(&mut self.context, "SM2M-EMULATOR", timeout)?;
        match device_lookup {
            Some(device_lookup) => Ok(Some(Self::create_device(device_lookup)?)),
            None => Ok(None),
        }
    }

    pub fn find_decoder(
        &mut self,
        timeout: time::Duration,
    ) -> Result<Option<UsbDevice>, DriverError> {
        let device_lookup =
            device_lookup_helper::find_device(&mut self.context, "SM2M-DECODER", timeout)?;
        match device_lookup {
            Some(device_lookup) => Ok(Some(Self::create_device(device_lookup)?)),
            None => Ok(None),
        }
    }

    pub fn find_encoder(
        &mut self,
        timeout: time::Duration,
    ) -> Result<Option<UsbDevice>, DriverError> {
        let device_lookup =
            device_lookup_helper::find_device(&mut self.context, "SM2M-ENCODER", timeout)?;
        match device_lookup {
            Some(device_lookup) => Ok(Some(Self::create_device(device_lookup)?)),
            None => Ok(None),
        }
    }

    fn create_device(
        mut device_lookup: DeviceLookup<rusb::Context>,
    ) -> Result<UsbDevice, DriverError> {
        let readable_endpoint = device_lookup.find_readable_endpoint()?;
        let writeable_endpoint = device_lookup.find_writeable_endpoint()?;
        UsbDevice::from(device_lookup, readable_endpoint, writeable_endpoint)
    }
}
