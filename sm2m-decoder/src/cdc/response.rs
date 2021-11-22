use usb_device::UsbError;

use super::device::CdcDevice;

pub enum Response {
    Pong(u8, u8),
}

const OUTPUT_BUFFER_SIZE: usize = 64;

pub struct ResponseWriter {
    buf: [u8; OUTPUT_BUFFER_SIZE],
    len: usize,
    pos: usize,
}

impl Default for ResponseWriter {
    fn default() -> Self {
        Self {
            buf: [0u8; OUTPUT_BUFFER_SIZE],
            len: 0,
            pos: 0,
        }
    }
}

impl ResponseWriter {
    pub fn write(&mut self, device: &mut CdcDevice, response: Response) -> Result<(), UsbError> {
        self.cache(response);
        while self.pos < self.len {
            let size = device.write(&self.buf[self.pos..self.len])?;
            self.pos += size;
        }
        self.reset();
        Ok(())
    }

    pub fn cache(&mut self, value: Response) {
        match value {
            Response::Pong(version, payload) => {
                let mut response = 0x1u16;
                response |= (version as u16) << 4;
                response |= (payload as u16) << 12;
                self.cache_buf(&response.to_le_bytes());
            }
        }
    }

    fn cache_buf(&mut self, buf: &[u8]) {
        for byte in buf {
            self.buf[self.len] = *byte;
            self.len += 1;
        }
    }

    fn reset(&mut self) {
        self.pos = 0;
        self.len = 0;
    }
}
