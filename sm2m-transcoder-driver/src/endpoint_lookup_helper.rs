use crate::endpoint_lookup::EndpointLookup;

enum Direction {
    Input,
    Output,
}

pub fn find_readable_endpoint<T: rusb::UsbContext>(
    device: &mut rusb::Device<T>,
    descriptor: &rusb::DeviceDescriptor,
    transfer_type: rusb::TransferType,
) -> Option<EndpointLookup> {
    find_endpoint(device, descriptor, transfer_type, Direction::Input)
}

pub fn find_writeable_endpoint<T: rusb::UsbContext>(
    device: &mut rusb::Device<T>,
    descriptor: &rusb::DeviceDescriptor,
    transfer_type: rusb::TransferType,
) -> Option<EndpointLookup> {
    find_endpoint(device, descriptor, transfer_type, Direction::Output)
}

fn find_endpoint<T: rusb::UsbContext>(
    device: &mut rusb::Device<T>,
    descriptor: &rusb::DeviceDescriptor,
    transfer_type: rusb::TransferType,
    direction: Direction,
) -> Option<EndpointLookup> {
    for config_index in 0..descriptor.num_configurations() {
        match device.config_descriptor(config_index) {
            Ok(config_descriptor) => {
                for interface in config_descriptor.interfaces() {
                    for interface_descriptor in interface.descriptors() {
                        for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
                            if has_direction(&endpoint_descriptor, transfer_type, &direction) {
                                log::debug!("Endpoint found: {:?}", transfer_type);
                                return Some(EndpointLookup::new(
                                    config_descriptor.number(),
                                    interface_descriptor.interface_number(),
                                    interface_descriptor.setting_number(),
                                    endpoint_descriptor.address(),
                                    transfer_type,
                                ));
                            }
                        }
                    }
                }
            }
            Err(error) => log::warn!(
                "Can't read device {}:{} configuration descriptor, reason: {:?}",
                descriptor.vendor_id(),
                descriptor.product_id(),
                error
            ),
        }
    }

    None
}

fn has_direction(
    descriptor: &rusb::EndpointDescriptor,
    transfer_type: rusb::TransferType,
    direction: &Direction,
) -> bool {
    let has_direction = match direction {
        Direction::Input => descriptor.direction() == rusb::Direction::In,
        Direction::Output => descriptor.direction() == rusb::Direction::Out,
    };

    has_direction && descriptor.transfer_type() == transfer_type
}
