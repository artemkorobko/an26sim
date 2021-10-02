use std::time;

use crate::{device_lookup::DeviceLookup, error::DriverError};

const VID: u16 = 1155;
const PID: u16 = 22336;

pub fn find_encoder<T: rusb::UsbContext>(
    context: &mut T,
    timeout: time::Duration,
) -> Result<Option<DeviceLookup<T>>, DriverError> {
    find_device(context, "SM2M-ENCODER", timeout)
}

pub fn find_decoder<T: rusb::UsbContext>(
    context: &mut T,
    timeout: time::Duration,
) -> Result<Option<DeviceLookup<T>>, DriverError> {
    find_device(context, "SM2M-DECODER", timeout)
}

fn find_device<T: rusb::UsbContext>(
    context: &mut T,
    serial_number: &str,
    timeout: time::Duration,
) -> Result<Option<DeviceLookup<T>>, DriverError> {
    log::debug!("Search for supported devices");
    let devices = context.devices().map_err(DriverError::DeviceList)?;
    for device in devices.iter() {
        let descriptor = device.device_descriptor();
        match descriptor {
            Ok(descriptor) => {
                if is_expected_device(&descriptor) {
                    let handle = device.open();
                    match handle {
                        Ok(handle) => {
                            if have_serial_number(&handle, &descriptor, serial_number, timeout)? {
                                log::debug!("Device {}:{} {} found", descriptor.vendor_id(), descriptor.product_id(), serial_number);
                                return Ok(Some(DeviceLookup::new(device, handle, descriptor)));
                            }
                        }
                        Err(error) => {
                            log::error!(
                                "Can't open device {}:{}, reason: {:?}",
                                descriptor.vendor_id(),
                                descriptor.product_id(),
                                error
                            )
                        }
                    }
                }
            }
            Err(error) => log::error!("Can't read device descriptor, reason: {:?}", error),
        }
    }

    Ok(None)
}

fn is_expected_device(descriptor: &rusb::DeviceDescriptor) -> bool {
    descriptor.vendor_id() == VID && descriptor.product_id() == PID
}

fn have_serial_number<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descriptor: &rusb::DeviceDescriptor,
    serial_number: &str,
    timeout: time::Duration,
) -> Result<bool, DriverError> {
    if let Some(language) = read_first_language(handle, descriptor, timeout) {
        let device_serial_number = handle
            .read_serial_number_string(language, descriptor, timeout)
            .map_err(|error| {
                DriverError::SerialNumber(error, descriptor.vendor_id(), descriptor.product_id())
            })?;
        Ok(serial_number.eq(&device_serial_number))
    } else {
        log::warn!(
            "Device {}:{} has no languages",
            descriptor.vendor_id(),
            descriptor.product_id()
        );
        Ok(false)
    }
}

fn read_first_language<T: rusb::UsbContext>(
    handle: &rusb::DeviceHandle<T>,
    descriptor: &rusb::DeviceDescriptor,
    timeout: time::Duration,
) -> Option<rusb::Language> {
    let languages = handle.read_languages(timeout);
    match languages {
        Ok(languages) => {
            if languages.is_empty() {
                None
            } else {
                Some(languages[0])
            }
        }
        Err(error) => {
            log::warn!(
                "Can't read languages {}:{}, reason: {:?}",
                descriptor.vendor_id(),
                descriptor.product_id(),
                error
            );
            None
        }
    }
}
