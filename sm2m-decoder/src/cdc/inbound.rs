use usb_device::UsbError;

use super::device::CdcDevice;

pub enum UsbInPacket {
    GetVersion,
    Ping(u8, u8),
    LedOn,
    LedOff,
    SetParam(u8, u16),
    GetParam(u8),
    EnableParamGenerator(u8, u8, u16),
    DisableParamGenerator(u8),
}

pub trait PacketReader {
    fn read_packet(&mut self) -> Result<Option<UsbInPacket>, UsbError>;
}

impl PacketReader for CdcDevice {
    fn read_packet(&mut self) -> Result<Option<UsbInPacket>, UsbError> {
        let mut buf = [0u8; 64];
        self.read(&mut buf)?;
        let packet = match opcode(&buf) {
            1 => Some(UsbInPacket::GetVersion),
            2 => Some(ping(&buf)),
            3 => Some(set_led_state(&buf)),
            4 => Some(set_param(&buf)),
            5 => Some(get_param(&buf)),
            6 => Some(enable_param_generator_props(&buf)),
            7 => Some(disable_param_generator(&buf)),
            // 8 => enable global generator
            // 9 => disable global generator
            // 10 => enable sm2m interrupt
            // 11 => disable sm2m interrupt
            _ => None,
        };
        Ok(packet)
    }
}

fn opcode(buf: &[u8]) -> u8 {
    buf[0] & 0x0f
}

fn ping(buf: &[u8]) -> UsbInPacket {
    let payload = buf[0] >> 4 & 0xf;
    let version = buf[1];
    UsbInPacket::Ping(version, payload)
}

fn set_led_state(buf: &[u8]) -> UsbInPacket {
    let state = buf[0] >> 4 & 1;
    if state == 0 {
        UsbInPacket::LedOff
    } else {
        UsbInPacket::LedOn
    }
}

fn set_param(buf: &[u8]) -> UsbInPacket {
    let index = buf[0] >> 4;
    let param = buf[1] as u16 | (buf[2] as u16) << 8;
    UsbInPacket::SetParam(index, param)
}

fn get_param(buf: &[u8]) -> UsbInPacket {
    let index = buf[0] >> 4;
    UsbInPacket::GetParam(index)
}

fn enable_param_generator_props(buf: &[u8]) -> UsbInPacket {
    let index = buf[0] >> 4;
    let step = buf[1] as u16 | (buf[2] as u16) << 8;
    let period = buf[3] >> 4;
    UsbInPacket::EnableParamGenerator(index, period, step)
}

fn disable_param_generator(buf: &[u8]) -> UsbInPacket {
    let index = buf[0] >> 4;
    UsbInPacket::DisableParamGenerator(index)
}
