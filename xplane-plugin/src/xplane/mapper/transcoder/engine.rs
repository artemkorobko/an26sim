use crate::shared::percent::Percent;

pub fn decode(value: u16) -> f32 {
    (value as f32).scale(u16::MIN as f32, u16::MAX as f32, 0.0, 166.0)
}

pub fn encode(value: f32) -> u16 {
    value
        .scale(0.0, 166.0, u16::MIN as f32, u16::MAX as f32)
        .round() as u16
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn decode_param() {
        assert_float_eq!(decode(u16::MIN), 0.0, abs <= 0.001);
        assert_float_eq!(decode(u16::MAX / 2), 83.0, abs <= 0.01);
        assert_float_eq!(decode(u16::MAX), 166.0, abs <= 0.001);
    }

    #[test]
    fn encode_param() {
        assert_eq!(encode(0.0), u16::MIN);
        assert_eq!(encode(83.0), u16::MAX / 2);
        assert_eq!(encode(166.0), u16::MAX);
    }
}
