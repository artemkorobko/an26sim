use usb_device::UsbError;

use super::device::CdcDevice;

pub enum Response {
    Pong(u8, u16),
}

const WRITE_BUFFER_SIZE: usize = 50;

pub struct CdcWriter {
    buffer: [u8; WRITE_BUFFER_SIZE],
    length: usize,
    offset: usize,
}

impl Default for CdcWriter {
    fn default() -> Self {
        Self {
            buffer: [0; WRITE_BUFFER_SIZE],
            length: 0,
            offset: 0,
        }
    }
}

impl CdcWriter {
    pub fn cache(&mut self, value: Response) {
        match value {
            Response::Pong(version, payload) => {
                self.cache_opcode(1);
                self.cache_bytes(&version.to_le_bytes());
                self.cache_bytes(&payload.to_le_bytes());
            }
        }
    }

    pub fn write(&mut self, device: &mut CdcDevice) -> Result<bool, UsbError> {
        if self.is_empty() {
            Ok(true)
        } else {
            let size = device.write(&mut self.buffer[self.offset..self.length])?;
            self.offset += size;
            if self.offset >= self.length {
                self.reset();
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    fn cache_opcode(&mut self, value: u8) {
        self.buffer[self.length] = value;
        self.length += 1;
    }

    fn cache_bytes(&mut self, value: &[u8]) {
        for byte in value {
            self.buffer[self.length] = *byte;
            self.length += 1;
        }
    }

    fn reset(&mut self) {
        self.offset = 0;
        self.length = 0;
    }

    fn is_empty(&self) -> bool {
        self.offset == 0 && self.length == 0
    }
}
