use usb_device::UsbError;

use super::device::CdcDevice;

pub enum UsbInbound {
    GetVersion,
    UpdateParam(u8, u16),
    EnableGenerator(u8, u8, u16),
    DisableGenerator(u8),
    StartGenerator(u8, u16),
    StopGenerator,
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
            2 => Some(update_param(&buf)),
            3 => Some(enable_generator(&buf)),
            4 => Some(disable_generator(&buf)),
            5 => Some(start_generator(&buf)),
            6 => Some(UsbInbound::StopGenerator),
            _ => None,
        })
    }
}

fn update_param(buf: &[u8]) -> UsbInbound {
    let index = buf[1];
    let value = buf[2] as u16 | (buf[3] as u16) << 8;
    UsbInbound::UpdateParam(index, value)
}

fn enable_generator(buf: &[u8]) -> UsbInbound {
    let index = buf[1];
    let period = buf[2];
    let step = buf[3] as u16 | (buf[4] as u16) << 8;
    UsbInbound::EnableGenerator(index, period, step)
}

fn disable_generator(buf: &[u8]) -> UsbInbound {
    let index = buf[1];
    UsbInbound::DisableGenerator(index)
}

fn start_generator(buf: &[u8]) -> UsbInbound {
    let millis = buf[1];
    let marker = buf[2] as u16 | (buf[3] as u16) << 8;
    UsbInbound::StartGenerator(millis, marker)
}
