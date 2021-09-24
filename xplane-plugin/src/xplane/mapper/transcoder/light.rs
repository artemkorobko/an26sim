pub fn decode(value: u16) -> (bool, bool, bool) {
    let landing = value & 0b1 == 1;
    let navigation = (value >> 1) & 0b1 == 1;
    let beacon = (value >> 2) & 0b1 == 1;
    (landing, navigation, beacon)
}

pub fn encode(landing: bool, navigation: bool, beacon: bool) -> u16 {
    let mut value = 0;

    if landing {
        value |= 1;
    }

    if navigation {
        value |= 2;
    }

    if beacon {
        value |= 4;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_param() {
        assert_eq!(decode(1), (true, false, false));
        assert_eq!(decode(2), (false, true, false));
        assert_eq!(decode(4), (false, false, true));
        assert_eq!(decode(7), (true, true, true));
    }

    #[test]
    fn encode_param() {
        assert_eq!(encode(true, false, false), 1);
        assert_eq!(encode(false, true, false), 2);
        assert_eq!(encode(false, false, true), 4);
        assert_eq!(encode(true, true, true), 7);
    }
}
