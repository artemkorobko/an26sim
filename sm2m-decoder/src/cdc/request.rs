use usb_device::UsbError;

use super::device::CdcDevice;

pub enum Request {
    Ping(u8, u8),
    LedOn,
    LedOff,
    SetParam(u8, u16),
    Unknown,
}

const INPUT_BUFFER_SIZE: usize = 64;

pub struct RequestReader {
    buf: [u8; INPUT_BUFFER_SIZE],
    len: usize,
}

impl Default for RequestReader {
    fn default() -> Self {
        Self {
            buf: [0u8; INPUT_BUFFER_SIZE],
            len: 0,
        }
    }
}

impl RequestReader {
    pub fn read_from(&mut self, device: &mut CdcDevice) -> Result<Option<Request>, UsbError> {
        let size = device.read(&mut self.buf[self.len..])?;
        self.len += size;
        let request = self.read().map(|request| {
            self.len = 0;
            request
        });
        Ok(request)
    }

    fn read(&mut self) -> Option<Request> {
        self.opcode().and_then(|opcode| match opcode {
            1 => self.safe_read(2, || self.ping()),
            2 => Some(self.led()),
            _ => Some(Request::Unknown),
        })
    }

    fn opcode(&self) -> Option<u8> {
        if self.len > 0 {
            let opcode = self.buf[0] & 0x0f;
            Some(opcode)
        } else {
            None
        }
    }

    fn ping(&self) -> Request {
        let version = self.buf[1] << 4 | self.buf[0] >> 4;
        let payload = self.buf[1] >> 4 & 0xf;
        Request::Ping(version, payload)
    }

    fn led(&self) -> Request {
        if (self.buf[0] >> 4) & 1 == 0 {
            Request::LedOff
        } else {
            Request::LedOn
        }
    }

    fn safe_read(&self, size: usize, fun: impl FnOnce() -> Request) -> Option<Request> {
        if self.buf.len() >= size {
            Some(fun())
        } else {
            None
        }
    }
}
