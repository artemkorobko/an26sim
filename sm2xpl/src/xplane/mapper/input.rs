use bytes::Bytes;

use crate::{
    common::{bytes::BytesExt, chain::Mapper, percent::Percent},
    xplane::{input_params::XPlaneInputParams, mapper::bit::bit_test},
};

#[derive(Default)]
pub struct SM2MXPlaneInputMapper;

impl SM2MXPlaneInputMapper {
    fn map_params(&self, input: Vec<u8>) -> XPlaneInputParams {
        let mut params = XPlaneInputParams::default();
        let mut buffer = Bytes::from(input);

        if let Some(value) = buffer.try_get_u32() {
            params.latitude = Self::latitude(value);
        }

        if let Some(value) = buffer.try_get_u32() {
            params.longitude = (value as f64).scale(0.0, u32::MAX as f64, 0.0, 360.0);
        }

        if let Some(value) = buffer.try_get_u16() {
            params.altitude = value as f64;
        }

        if let Some(value) = buffer.try_get_i16() {
            params.heading = (value as f32).scale(0.0, 32767.0, 0.0, 359.99);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.pitch = (value as f32).scale(-32767.0, 32767.0, -45.0, 45.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.roll = (value as f32).scale(-32767.0, 32767.0, -90.0, 90.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.ailerons = (value as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.elevator = (value as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.rudder = (value as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.flaps = (value as f32).scale(0.0, 32767.0, 0.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.engine_left = (value as f32).scale(0.0, 32767.0, 0.0, 166.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.engine_right = (value as f32).scale(0.0, 32767.0, 0.0, 166.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.gear_front = (value as f32).scale(0.0, 32767.0, 0.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.gear_left = (value as f32).scale(0.0, 32767.0, 0.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.gear_right = (value as f32).scale(0.0, 32767.0, 0.0, 1.0);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.light_landing = bit_test(value, 0);
            params.light_navigation = bit_test(value, 1);
            params.light_beacon = bit_test(value, 2);
        }

        if let Some(value) = buffer.try_get_i16() {
            params.reset = bit_test(value, 0);
        }

        params
    }

    fn latitude(value: u32) -> f64 {
        (value as f64).scale(0.0, u32::MAX as f64, 0.0, 90.0)
    }
}

impl Mapper<Option<Vec<u8>>, Option<XPlaneInputParams>> for SM2MXPlaneInputMapper {
    fn map(&mut self, input: Option<Vec<u8>>) -> Option<XPlaneInputParams> {
        input.and_then(|params| Some(self.map_params(params)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use float_eq::assert_float_eq;

    #[test]
    fn should_map_default_params() {
        let mut mapper = SM2MXPlaneInputMapper::default();

        let params = mapper.map(Some(Vec::new())).unwrap();

        assert_eq!(params, Default::default());
    }

    #[test]
    fn should_return_none_when_input_params_are_absent() {
        let mut mapper = default_mapper();

        let params = mapper.map(None);

        assert!(params.is_none());
    }

    #[test]
    fn should_map_latitude() {
        let input = input_with_u32_param(2402881062, 0);
        let mut mapper = default_mapper();

        let params = mapper.map(Some(input)).unwrap();

        assert_float_eq!(params.latitude, 50.351791, abs <= 0.00001);
    }

    #[test]
    fn should_map_longitude() {
        let input = input_with_u32_param(368431135, 4);
        let mut mapper = default_mapper();

        let params = mapper.map(Some(input)).unwrap();

        assert_float_eq!(params.longitude, 30.881541, abs <= 0.00001);
    }

    #[test]
    fn should_map_altitude() {
        let pos = 8;
        let mut mapper = default_mapper();

        let input = input_with_u16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.altitude, 0.0, abs <= 0.001);

        let input = input_with_u16_param(4000, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.altitude, 4000.0, abs <= 0.001);

        let input = input_with_u16_param(8000, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.altitude, 8000.0, abs <= 0.001);
    }

    #[test]
    fn should_map_heading() {
        let pos = 10;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.heading, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.heading, 180.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.heading, 359.99, abs <= 0.001);
    }

    #[test]
    fn should_map_pitch() {
        let pos = 12;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(-32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.pitch, -45.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.pitch, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.pitch, 45.0, abs <= 0.001);
    }

    #[test]
    fn should_map_roll() {
        let pos = 14;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(-32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.roll, -90.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.roll, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.roll, 90.0, abs <= 0.001);
    }

    #[test]
    fn should_map_ailerons() {
        let pos = 16;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(-32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.ailerons, -1.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.ailerons, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.ailerons, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_elevator() {
        let pos = 18;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(-32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.elevator, -1.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.elevator, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.elevator, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_rudder() {
        let pos = 20;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(-32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.rudder, -1.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.rudder, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.rudder, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_flaps() {
        let pos = 22;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.flaps, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.flaps, 0.5, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.flaps, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_engine_left() {
        let pos = 24;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.engine_left, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.engine_left, 83.0, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.engine_left, 166.0, abs <= 0.001);
    }

    #[test]
    fn should_map_engine_right() {
        let pos = 26;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.engine_right, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.engine_right, 83.0, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.engine_right, 166.0, abs <= 0.001);
    }

    #[test]
    fn should_map_gear_front() {
        let pos = 28;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_front, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_front, 0.5, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_front, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_gear_left() {
        let pos = 30;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_left, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_left, 0.5, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_left, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_gear_right() {
        let pos = 32;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_right, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_right, 0.5, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_float_eq!(params.gear_right, 1.0, abs <= 0.001);
    }

    #[test]
    fn should_map_light_landing() {
        let pos = 34;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.light_landing, false);

        let input = input_with_i16_param(1, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.light_landing, true);
    }

    #[test]
    fn should_map_light_navigation() {
        let pos = 34;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(1, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.light_navigation, false);

        let input = input_with_i16_param(2, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.light_navigation, true);
    }

    #[test]
    fn should_map_light_beacon() {
        let pos = 34;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(3, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.light_beacon, false);

        let input = input_with_i16_param(4, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.light_beacon, true);
    }

    #[test]
    fn should_map_reset() {
        let pos = 36;
        let mut mapper = default_mapper();

        let input = input_with_i16_param(0, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.reset, false);

        let input = input_with_i16_param(1, pos);
        let params = mapper.map(Some(input)).unwrap();
        assert_eq!(params.reset, true);
    }

    fn default_mapper() -> SM2MXPlaneInputMapper {
        SM2MXPlaneInputMapper::default()
    }

    fn input_with_i16_param(value: i16, pos: usize) -> Vec<u8> {
        input_with_buffer(&value.to_be_bytes(), pos)
    }

    fn input_with_u16_param(value: u16, pos: usize) -> Vec<u8> {
        input_with_buffer(&value.to_be_bytes(), pos)
    }

    fn input_with_u32_param(value: u32, pos: usize) -> Vec<u8> {
        input_with_buffer(&value.to_be_bytes(), pos)
    }

    fn input_with_buffer(buffer: &[u8], pos: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(pos + buffer.len());
        if pos > 0 {
            vec.extend(vec![0; pos]);
        }
        vec.extend(buffer);
        vec
    }
}
