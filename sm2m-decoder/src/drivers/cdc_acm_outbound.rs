use usb_device::UsbError;

use crate::params::MAX_PARAMS_COUNT;

use super::cdc_acm::{Device, MAX_PACKET_SIZE};

pub enum Outbound {
    FirmwareVersion(u8, u8, u8),
    Params([u16; MAX_PARAMS_COUNT], usize),
}

pub trait Writer {
    fn write_outbound(&mut self, packet: Outbound) -> Result<usize, UsbError>;
}

impl Writer for Device {
    fn write_outbound(&mut self, packet: Outbound) -> Result<usize, UsbError> {
        match packet {
            Outbound::FirmwareVersion(major, minor, patch) => {
                let buf = [1, major, minor, patch];
                self.write_all(&buf)
            }
            Outbound::Params(params, count) => {
                let params_count = count as u8;
                if params_count > MAX_PACKET_SIZE {
                    send_params_overflow(self, MAX_PACKET_SIZE, params_count)
                } else {
                    send_params(self, params, count)
                }
            }
        }
    }
}

fn send_params(
    device: &mut Device,
    params: [u16; MAX_PARAMS_COUNT],
    count: usize,
) -> Result<usize, UsbError> {
    let mut buf = [0; MAX_PACKET_SIZE as usize];
    buf[0] = 2;
    buf[1] = 0;
    buf[2] = count as u8;
    let mut buf_idx = 3;
    for param in params.iter().take(count) {
        buf[buf_idx] = *param as u8;
        buf[buf_idx + 1] = (param >> 8) as u8;
        buf_idx += 2;
    }
    device.write_all(&buf[..=buf_idx])
}

fn send_params_overflow(
    device: &mut Device,
    expected: u8,
    received: u8,
) -> Result<usize, UsbError> {
    let buf = [2, 1, expected, received];
    device.write_all(&buf)
}
