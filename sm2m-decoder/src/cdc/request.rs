use usb_device::UsbError;

use super::device::CdcDevice;

pub enum Request {
    Ping(u8, u8),
    LedOn,
    LedOff,
    SetParam(u8, u16),
    GetParam(u8),
}

pub trait RequestReader {
    fn read_request(&mut self) -> Result<Option<Request>, UsbError>;
}

impl RequestReader for CdcDevice {
    fn read_request(&mut self) -> Result<Option<Request>, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let request = match read_opcode(&buf) {
            1 => Some(read_ping(&buf)),
            2 => Some(read_led(&buf)),
            3 => Some(read_set_param(&buf)),
            4 => Some(read_get_param(&buf)),
            _ => None,
        };
        Ok(request)
    }
}

fn read_opcode(buf: &[u8]) -> u8 {
    buf[0] & 0x0f
}

fn read_ping(buf: &[u8]) -> Request {
    let payload = buf[0] >> 4 & 0xf;
    let version = buf[1];
    Request::Ping(version, payload)
}

fn read_led(buf: &[u8]) -> Request {
    let state = buf[0] >> 4 & 1;
    if state == 0 {
        Request::LedOff
    } else {
        Request::LedOn
    }
}

fn read_set_param(buf: &[u8]) -> Request {
    let index = buf[0] >> 4;
    let param = buf[1] as u16 | (buf[2] as u16) << 8;
    Request::SetParam(index, param)
}

fn read_get_param(buf: &[u8]) -> Request {
    let index = buf[0] >> 4;
    Request::GetParam(index)
}
