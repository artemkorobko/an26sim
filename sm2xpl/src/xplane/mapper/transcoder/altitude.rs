pub fn decode(value: i16) -> f64 {
    value as f64
}

pub fn encode(value: f64) -> i16 {
    value.round() as i16
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn decode_param() {
        assert_float_eq!(decode(1000), 1000.0, abs <= 0.01);
        assert_float_eq!(decode(4000), 4000.0, abs <= 0.01);
        assert_float_eq!(decode(8000), 8000.0, abs <= 0.01);
    }

    #[test]
    fn encode_param() {
        assert_eq!(encode(8765.10), 8765);
        assert_eq!(encode(8765.50), 8766);
        assert_eq!(encode(8765.80), 8766);
    }
}
