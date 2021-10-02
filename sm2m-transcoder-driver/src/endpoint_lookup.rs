use std::time;

use crate::error::DriverError;

pub struct EndpointLookup {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
    transfer_type: rusb::TransferType,
    driver_detached: bool,
}

impl EndpointLookup {
    pub fn new(
        config: u8,
        iface: u8,
        setting: u8,
        address: u8,
        transfer_type: rusb::TransferType,
    ) -> Self {
        Self {
            config,
            iface,
            setting,
            address,
            transfer_type,
            driver_detached: false,
        }
    }

    pub fn read<T: rusb::UsbContext>(
        &mut self,
        handle: &rusb::DeviceHandle<T>,
        buf: &mut [u8],
        timeout: time::Duration,
    ) -> Result<usize, DriverError> {
        match self.transfer_type {
            rusb::TransferType::Bulk => handle
                .read_bulk(self.address, buf, timeout)
                .map_err(|error| DriverError::Read(error, self.address)),
            rusb::TransferType::Interrupt => handle
                .read_interrupt(self.address, buf, timeout)
                .map_err(|error| DriverError::Read(error, self.address)),
            _ => Err(DriverError::UnsupportedInputTransferType(
                self.transfer_type,
                self.address,
            )),
        }
    }

    pub fn write<T: rusb::UsbContext>(
        &mut self,
        handle: &rusb::DeviceHandle<T>,
        buf: &[u8],
        timeout: time::Duration,
    ) -> Result<usize, DriverError> {
        match self.transfer_type {
            rusb::TransferType::Bulk => handle
                .write_bulk(self.address, buf, timeout)
                .map_err(|error| DriverError::Write(error, self.address)),
            rusb::TransferType::Interrupt => handle
                .write_interrupt(self.address, buf, timeout)
                .map_err(|error| DriverError::Write(error, self.address)),
            _ => Err(DriverError::UnsupportedOutputTransferType(
                self.transfer_type,
                self.address,
            )),
        }
    }

    pub fn open<T: rusb::UsbContext>(
        &mut self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        if self.is_kernel_driver_active(handle)? {
            log::debug!("Detach kernel driver");
            self.detach_kernel_driver(handle)?;
            self.driver_detached = true;
        }

        self.set_active_configuration(handle)?;
        self.claim_interface(handle)?;
        self.set_alternate_setting(handle)?;
        Ok(())
    }

    pub fn close<T: rusb::UsbContext>(
        &mut self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        if self.driver_detached {
            log::debug!("Attach kernel driver");
            self.attach_kernel_driver(handle)?;
        }

        Ok(())
    }

    fn is_kernel_driver_active<T: rusb::UsbContext>(
        &self,
        handle: &rusb::DeviceHandle<T>,
    ) -> Result<bool, DriverError> {
        handle
            .kernel_driver_active(self.iface)
            .map_err(|error| DriverError::KernelDriverState(error, self.iface))
    }

    fn detach_kernel_driver<T: rusb::UsbContext>(
        &mut self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        handle
            .detach_kernel_driver(self.iface)
            .map_err(|error| DriverError::DetachKernelDriver(error, self.iface))?;
        self.driver_detached = true;
        Ok(())
    }

    fn attach_kernel_driver<T: rusb::UsbContext>(
        &mut self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        if self.driver_detached {
            handle
                .attach_kernel_driver(self.iface)
                .map_err(|error| DriverError::AttachKernelDriver(error, self.iface))?;
            self.driver_detached = false;
        }
        Ok(())
    }

    fn set_active_configuration<T: rusb::UsbContext>(
        &self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        handle
            .set_active_configuration(self.config)
            .map_err(|error| DriverError::ActiveConfiguration(error, self.config))
    }

    fn claim_interface<T: rusb::UsbContext>(
        &self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        handle
            .claim_interface(self.iface)
            .map_err(|error| DriverError::ClaimInterface(error, self.iface))
    }

    fn set_alternate_setting<T: rusb::UsbContext>(
        &self,
        handle: &mut rusb::DeviceHandle<T>,
    ) -> Result<(), DriverError> {
        handle
            .set_alternate_setting(self.iface, self.setting)
            .map_err(|error| DriverError::AlternateSetting(error, self.setting, self.iface))
    }
}
