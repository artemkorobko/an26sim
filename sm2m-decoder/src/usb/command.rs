pub enum USBCommand {
    Ping,
    Pong,
    Unknown,
}

impl From<u8> for USBCommand {
    fn from(value: u8) -> Self {
        match value {
            1 => USBCommand::Ping,
            2 => USBCommand::Pong,
            _ => USBCommand::Unknown,
        }
    }
}

impl USBCommand {
    pub fn to_u8(&self) -> u8 {
        match self {
            USBCommand::Ping => 1,
            USBCommand::Pong => 2,
            USBCommand::Unknown => 255,
        }
    }
}
