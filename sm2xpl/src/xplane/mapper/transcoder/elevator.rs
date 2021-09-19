use crate::common::percent::Percent;

pub fn decode(value: i16) -> f32 {
    (value as f32).scale(i16::MIN as f32, i16::MAX as f32, -1.0, 1.0)
}

pub fn encode(value: f32) -> i16 {
    value
        .scale(-1.0, 1.0, i16::MIN as f32, i16::MAX as f32)
        .round() as i16
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn decode_param() {
        assert_float_eq!(decode(i16::MIN), -1.0, abs <= 0.001);
        assert_float_eq!(decode(0), 0.0, abs <= 0.01);
        assert_float_eq!(decode(i16::MAX), 1.0, abs <= 0.001);
    }

    #[test]
    fn encode_param() {
        assert_eq!(encode(-1.0), i16::MIN);
        assert_eq!(encode(0.0), -1);
        assert_eq!(encode(1.0), i16::MAX);
    }
}
