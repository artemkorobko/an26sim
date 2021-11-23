use usb_device::UsbError;

use super::device::CdcDevice;

pub enum Response {
    Pong(u8, u8),
    Param(u8, u16),
}

pub trait ResponseWriter {
    fn write_response(&mut self, response: Response) -> Result<usize, UsbError>;
}

impl ResponseWriter for CdcDevice {
    fn write_response(&mut self, response: Response) -> Result<usize, UsbError> {
        match response {
            Response::Pong(version, payload) => {
                let buf = [1 | payload << 4, version];
                write_all(self, &buf)
            }
            Response::Param(index, param) => {
                let buf = [4 | index << 4, param as u8, (param >> 8) as u8];
                write_all(self, &buf)
            }
        }
    }
}

fn write_all(device: &mut CdcDevice, buf: &[u8]) -> Result<usize, UsbError> {
    let mut sent = 0;
    while sent < buf.len() {
        sent += device.write(buf)?;
    }
    Ok(sent)
}
