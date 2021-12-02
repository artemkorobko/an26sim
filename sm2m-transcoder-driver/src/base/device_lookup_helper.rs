use std::time;

use crate::error::DriverError;

use super::device_lookup::DeviceLookup;

const VID: u16 = 1155;
const PID: u16 = 22336;

pub fn find_device<T: rusb::UsbContext>(
    context: &mut T,
    serial_number: &str,
    timeout: time::Duration,
) -> Result<Option<DeviceLookup<T>>, DriverError> {
    let devices = context.devices().map_err(DriverError::DeviceList)?;
    for device in devices.iter() {
        let descriptor = read_device_descriptor(&device)?;
        if is_expected_device(&descriptor) {
            let handle = open_device(&device, &descriptor)?;
            if has_serial_number(&handle, &descriptor, serial_number, timeout)? {
                return Ok(Some(DeviceLookup::new(device, handle, descriptor)));
            }
        }
    }

    Ok(None)
}

fn read_device_descriptor<T: rusb::UsbContext>(
    device: &rusb::Device<T>,
) -> Result<rusb::DeviceDescriptor, DriverError> {
    device
        .device_descriptor()
        .map_err(DriverError::ReadDeviceDescriptor)
}

fn is_expected_device(descriptor: &rusb::DeviceDescriptor) -> bool {
    descriptor.vendor_id() == VID && descriptor.product_id() == PID
}

fn open_device<T: rusb::UsbContext>(
    device: &rusb::Device<T>,
    descriptor: &rusb::DeviceDescriptor,
) -> Result<rusb::DeviceHandle<T>, DriverError> {
    device.open().map_err(|error| {
        DriverError::OpenDevice(error, descriptor.vendor_id(), descriptor.product_id())
    })
}

fn has_serial_number<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descriptor: &rusb::DeviceDescriptor,
    serial_number: &str,
    timeout: time::Duration,
) -> Result<bool, DriverError> {
    let language = read_first_language(handle, descriptor, timeout)?;
    if let Some(language) = language {
        let device_serial = read_serial_number_string(handle, descriptor, language, timeout)?;
        Ok(device_serial.eq(serial_number))
    } else {
        Ok(false)
    }
}

fn read_serial_number_string<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descriptor: &rusb::DeviceDescriptor,
    language: rusb::Language,
    timeout: time::Duration,
) -> Result<String, DriverError> {
    handle
        .read_serial_number_string(language, descriptor, timeout)
        .map_err(|error| {
            DriverError::SerialNumber(error, descriptor.vendor_id(), descriptor.product_id())
        })
}

fn read_first_language<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descriptor: &rusb::DeviceDescriptor,
    timeout: time::Duration,
) -> Result<Option<rusb::Language>, DriverError> {
    let language = handle
        .read_languages(timeout)
        .map_err(|error| {
            DriverError::ReadLanguages(error, descriptor.vendor_id(), descriptor.product_id())
        })?
        .first()
        .cloned();
    Ok(language)
}
