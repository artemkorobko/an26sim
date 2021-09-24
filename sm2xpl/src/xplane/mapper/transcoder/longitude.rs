use crate::shared::percent::Percent;

pub fn decode(value: u32) -> f64 {
    (value as f64).scale(0.0, u32::MAX as f64, 0.0, 360.0)
}

pub fn encode(value: f64) -> u32 {
    value.scale(0.0, 360.0, 0.0, u32::MAX as f64).round() as u32
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn decode_param() {
        assert_float_eq!(decode(368431135), 30.881541, abs <= 0.00001);
    }

    #[test]
    fn encode_param() {
        assert_eq!(encode(30.881541), 368431135);
    }
}
