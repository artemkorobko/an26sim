use crate::error::DriverError;

use super::{endpoint_lookup::EndpointLookup, endpoint_lookup_helper};

pub struct DeviceLookup<T: rusb::UsbContext> {
    pub device: rusb::Device<T>,
    pub handle: rusb::DeviceHandle<T>,
    pub descriptor: rusb::DeviceDescriptor,
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
        self.find_readable_typed_endpoint(rusb::TransferType::Bulk)
            .or_else(|| self.find_readable_typed_endpoint(rusb::TransferType::Interrupt))
            .ok_or_else(|| {
                DriverError::NoReadableEndpoint(
                    self.descriptor.vendor_id(),
                    self.descriptor.product_id(),
                )
            })
    }

    pub fn find_writeable_endpoint(&mut self) -> Result<EndpointLookup, DriverError> {
        self.find_writeable_typed_endpoint(rusb::TransferType::Bulk)
            .or_else(|| self.find_writeable_typed_endpoint(rusb::TransferType::Interrupt))
            .ok_or_else(|| {
                DriverError::NoWriteableEndpoint(
                    self.descriptor.vendor_id(),
                    self.descriptor.product_id(),
                )
            })
    }

    fn find_readable_typed_endpoint(
        &mut self,
        transfer_type: rusb::TransferType,
    ) -> Option<EndpointLookup> {
        endpoint_lookup_helper::find_readable_endpoint(
            &mut self.device,
            &self.descriptor,
            transfer_type,
        )
    }

    fn find_writeable_typed_endpoint(
        &mut self,
        transfer_type: rusb::TransferType,
    ) -> Option<EndpointLookup> {
        endpoint_lookup_helper::find_writeable_endpoint(
            &mut self.device,
            &self.descriptor,
            transfer_type,
        )
    }
}
