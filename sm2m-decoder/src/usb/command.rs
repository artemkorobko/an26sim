pub enum USBCommand {
    Ping,
    Pong,
    None,
}

impl Default for USBCommand {
    fn default() -> Self {
        Self::None
    }
}

impl From<u8> for USBCommand {
    fn from(value: u8) -> Self {
        match value {
            1 => USBCommand::Ping,
            2 => USBCommand::Pong,
            _ => USBCommand::None,
        }
    }
}

impl USBCommand {
    pub fn to_u8(&self) -> u8 {
        match self {
            USBCommand::Ping => 1,
            USBCommand::Pong => 2,
            USBCommand::None => 255,
        }
    }
}
