use usb_device::UsbError;

use super::cdc_acm::Device;

pub enum Inbound {
    FirmwareVersion,
    EnableGenerator(u8, u8, u16, u16),
    DisableGenerator(u8),
    StartProducer(u8),
    StopProducer,
}

pub trait Reader {
    fn read_inbound(&mut self) -> Result<Option<Inbound>, UsbError>;
}

impl Reader for Device {
    fn read_inbound(&mut self) -> Result<Option<Inbound>, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let opcode = buf[0];
        Ok(match opcode {
            1 => Some(Inbound::FirmwareVersion),
            2 => Some(enable_generator(&buf)),
            3 => Some(disable_generator(&buf)),
            4 => Some(start_producer(&buf)),
            5 => Some(Inbound::StopProducer),
            _ => None,
        })
    }
}

fn enable_generator(buf: &[u8]) -> Inbound {
    let index = buf[1];
    let period = buf[2];
    let value = buf[3] as u16 | (buf[4] as u16) << 8;
    let step = buf[5] as u16 | (buf[6] as u16) << 8;
    Inbound::EnableGenerator(index, period, value, step)
}

fn disable_generator(buf: &[u8]) -> Inbound {
    let index = buf[1];
    Inbound::DisableGenerator(index)
}

fn start_producer(buf: &[u8]) -> Inbound {
    let fps = buf[1];
    Inbound::StartProducer(fps)
}
