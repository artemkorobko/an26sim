use crate::{endpoint_lookup::EndpointLookup, endpoint_lookup_helper, error::DriverError};

pub struct DeviceLookup<T: rusb::UsbContext> {
    device: rusb::Device<T>,
    handle: rusb::DeviceHandle<T>,
    descriptor: rusb::DeviceDescriptor,
}

impl<T: rusb::UsbContext> DeviceLookup<T> {
    pub fn new(
        device: rusb::Device<T>,
        handle: rusb::DeviceHandle<T>,
        descriptor: rusb::DeviceDescriptor,
    ) -> Self {
        Self {
            device,
            handle,
            descriptor,
        }
    }

    pub fn find_readable_endpoint(&mut self) -> Result<EndpointLookup, DriverError> {
        let endpoint = endpoint_lookup_helper::find_readable_endpoint(
            &mut self.device,
            &self.descriptor,
            rusb::TransferType::Bulk,
        );

        if let Some(endpoint) = endpoint {
            return Ok(endpoint);
        }

        let endpoint = endpoint_lookup_helper::find_readable_endpoint(
            &mut self.device,
            &self.descriptor,
            rusb::TransferType::Interrupt,
        );

        if let Some(endpoint) = endpoint {
            return Ok(endpoint);
        }

        Err(DriverError::NoReadableEndpoint(
            self.descriptor.vendor_id(),
            self.descriptor.product_id(),
        ))
    }

    pub fn find_writeable_endpoint(&mut self) -> Result<EndpointLookup, DriverError> {
        let endpoint = endpoint_lookup_helper::find_writeable_endpoint(
            &mut self.device,
            &self.descriptor,
            rusb::TransferType::Bulk,
        );

        if let Some(endpoint) = endpoint {
            return Ok(endpoint);
        }

        let endpoint = endpoint_lookup_helper::find_writeable_endpoint(
            &mut self.device,
            &self.descriptor,
            rusb::TransferType::Interrupt,
        );

        if let Some(endpoint) = endpoint {
            return Ok(endpoint);
        }

        Err(DriverError::NoWriteableEndpoint(
            self.descriptor.vendor_id(),
            self.descriptor.product_id(),
        ))
    }

    pub fn device(&mut self) -> &mut rusb::Device<T> {
        &mut self.device
    }

    pub fn handle(&mut self) -> &mut rusb::DeviceHandle<T> {
        &mut self.handle
    }

    pub fn descriptor(&mut self) -> &mut rusb::DeviceDescriptor {
        &mut self.descriptor
    }
}
