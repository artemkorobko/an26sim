pub fn decode(value: u16) -> bool {
    value & 0b1 == 1
}

pub fn encode(value: bool) -> u16 {
    match value {
        true => 1,
        false => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_param() {
        assert_eq!(decode(0), false);
        assert_eq!(decode(1), true);
    }

    #[test]
    fn encode_param() {
        assert_eq!(encode(true), 1);
        assert_eq!(encode(false), 0);
    }
}
