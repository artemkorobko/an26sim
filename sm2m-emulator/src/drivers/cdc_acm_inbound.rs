use usb_device::UsbError;

use super::cdc_acm::CdcDevice;

pub enum UsbInbound {
    GetVersion,
    EnableGenerator(u8, u8, u16, u16),
    DisableGenerator(u8),
    StartProducer(u8),
    StopProducer,
}

pub trait Reader {
    fn read_ex(&mut self) -> Result<Option<UsbInbound>, UsbError>;
}

impl Reader for CdcDevice {
    fn read_ex(&mut self) -> Result<Option<UsbInbound>, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let opcode = buf[0];
        Ok(match opcode {
            1 => Some(UsbInbound::GetVersion),
            2 => Some(enable_generator(&buf)),
            3 => Some(disable_generator(&buf)),
            4 => Some(start_producer(&buf)),
            5 => Some(UsbInbound::StopProducer),
            _ => None,
        })
    }
}

fn enable_generator(buf: &[u8]) -> UsbInbound {
    let index = buf[1];
    let period = buf[2];
    let value = buf[3] as u16 | (buf[4] as u16) << 8;
    let step = buf[5] as u16 | (buf[6] as u16) << 8;
    UsbInbound::EnableGenerator(index, period, value, step)
}

fn disable_generator(buf: &[u8]) -> UsbInbound {
    let index = buf[1];
    UsbInbound::DisableGenerator(index)
}

fn start_producer(buf: &[u8]) -> UsbInbound {
    let fps = buf[1];
    UsbInbound::StartProducer(fps)
}
