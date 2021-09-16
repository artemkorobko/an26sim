use std::cell::Ref;

use bytes::Buf;

use crate::common::percent::Percent;

use super::dataref::collection::DataRefs;

const EXPECTED_BUF_BYTES: usize = 38;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct XPlaneInputParams {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub ailerons: f32,
    pub elevator: f32,
    pub rudder: f32,
    pub flaps: f32,
    pub engine_left: f32,
    pub engine_right: f32,
    pub gear_front: f32,
    pub gear_left: f32,
    pub gear_right: f32,
    pub light_landing: bool,
    pub light_navigation: bool,
    pub light_beacon: bool,
    pub reset: bool,
}

impl XPlaneInputParams {
    pub fn expected_buf_bytes() -> usize {
        EXPECTED_BUF_BYTES
    }
}

impl From<Ref<'_, DataRefs>> for XPlaneInputParams {
    fn from(datarefs: Ref<DataRefs>) -> Self {
        let coords = datarefs.location.coords();
        let engines = datarefs.engines.get();
        let gears = datarefs.gears.get();
        Self {
            latitude: coords.latitude,
            longitude: coords.longitude,
            altitude: coords.altitude,
            heading: datarefs.orientation.heading(),
            pitch: datarefs.orientation.pitch(),
            roll: datarefs.orientation.roll(),
            ailerons: datarefs.surfaces.ailerons(),
            elevator: datarefs.surfaces.elevator(),
            rudder: datarefs.surfaces.rudder(),
            flaps: datarefs.general.fps(),
            engine_left: engines.left,
            engine_right: engines.right,
            gear_front: gears.front,
            gear_left: gears.left,
            gear_right: gears.right,
            light_landing: datarefs.lights.landing(),
            light_navigation: datarefs.lights.navigation(),
            light_beacon: datarefs.lights.beacon(),
            reset: false,
        }
    }
}

impl From<&[u8]> for XPlaneInputParams {
    fn from(mut buffer: &[u8]) -> Self {
        let mut params = XPlaneInputParams::default();
        params.latitude = (buffer.get_u32() as f64).scale(0.0, u32::MAX as f64, 0.0, 90.0);
        params.longitude = (buffer.get_u32() as f64).scale(0.0, u32::MAX as f64, 0.0, 360.0);
        params.altitude = buffer.get_i16() as f64;
        params.heading = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 359.99);
        params.pitch = (buffer.get_i16() as f32).scale(-32767.0, 32767.0, -45.0, 45.0);
        params.roll = (buffer.get_i16() as f32).scale(-32767.0, 32767.0, -90.0, 90.0);
        params.ailerons = (buffer.get_i16() as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        params.elevator = (buffer.get_i16() as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        params.rudder = (buffer.get_i16() as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        params.flaps = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        params.engine_left = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 166.0);
        params.engine_right = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 166.0);
        params.gear_front = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        params.gear_left = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        params.gear_right = (buffer.get_i16() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        let param = buffer.get_u16();
        params.light_landing = param & 0b1 == 1;
        params.light_navigation = (param >> 1) & 0b1 == 1;
        params.light_beacon = (param >> 2) & 0b1 == 1;
        let param = buffer.get_u16();
        params.reset = param & 0b1 == 1;
        params
    }
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn map_latitude() {
        let input = input_with_u32_param(2402881062, 0);

        let params = XPlaneInputParams::from(input.as_ref());

        assert_float_eq!(params.latitude, 50.351791, abs <= 0.00001);
    }

    #[test]
    fn map_longitude() {
        let input = input_with_u32_param(368431135, 4);

        let params = XPlaneInputParams::from(input.as_ref());

        assert_float_eq!(params.longitude, 30.881541, abs <= 0.00001);
    }

    #[test]
    fn map_altitude() {
        let pos = 8;

        let input = input_with_u16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.altitude, 0.0, abs <= 0.001);

        let input = input_with_u16_param(4000, 8);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.altitude, 4000.0, abs <= 0.001);

        let input = input_with_u16_param(8000, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.altitude, 8000.0, abs <= 0.001);
    }

    #[test]
    fn map_heading() {
        let pos = 10;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.heading, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.heading, 180.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.heading, 359.99, abs <= 0.001);
    }

    #[test]
    fn map_pitch() {
        let pos = 12;

        let input = input_with_i16_param(-32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.pitch, -45.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.pitch, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.pitch, 45.0, abs <= 0.001);
    }

    #[test]
    fn map_roll() {
        let pos = 14;

        let input = input_with_i16_param(-32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.roll, -90.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.roll, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.roll, 90.0, abs <= 0.001);
    }

    #[test]
    fn map_ailerons() {
        let pos = 16;

        let input = input_with_i16_param(-32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.ailerons, -1.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.ailerons, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.ailerons, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_elevator() {
        let pos = 18;

        let input = input_with_i16_param(-32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.elevator, -1.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.elevator, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.elevator, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_rudder() {
        let pos = 20;

        let input = input_with_i16_param(-32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.rudder, -1.0, abs <= 0.001);

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.rudder, 0.0, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.rudder, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_flaps() {
        let pos = 22;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.flaps, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.flaps, 0.5, abs <= 0.001);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.flaps, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_engine_left() {
        let pos = 24;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.engine_left, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.engine_left, 83.0, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.engine_left, 166.0, abs <= 0.001);
    }

    #[test]
    fn map_engine_right() {
        let pos = 26;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.engine_right, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.engine_right, 83.0, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.engine_right, 166.0, abs <= 0.001);
    }

    #[test]
    fn map_gear_front() {
        let pos = 28;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_front, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_front, 0.5, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_front, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_gear_left() {
        let pos = 30;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_left, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_left, 0.5, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_left, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_gear_right() {
        let pos = 32;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_right, 0.0, abs <= 0.001);

        let input = input_with_i16_param(16384, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_right, 0.5, abs <= 0.005);

        let input = input_with_i16_param(32767, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_float_eq!(params.gear_right, 1.0, abs <= 0.001);
    }

    #[test]
    fn map_light_landing() {
        let pos = 34;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.light_landing, false);

        let input = input_with_i16_param(1, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.light_landing, true);
    }

    #[test]
    fn map_light_navigation() {
        let pos = 34;

        let input = input_with_i16_param(1, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.light_navigation, false);

        let input = input_with_i16_param(2, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.light_navigation, true);
    }

    #[test]
    fn map_light_beacon() {
        let pos = 34;

        let input = input_with_i16_param(3, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.light_beacon, false);

        let input = input_with_i16_param(4, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.light_beacon, true);
    }

    #[test]
    fn map_reset() {
        let pos = 36;

        let input = input_with_i16_param(0, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.reset, false);

        let input = input_with_i16_param(1, pos);
        let params = XPlaneInputParams::from(input.as_ref());
        assert_eq!(params.reset, true);
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
        let mut res_buf = vec![0u8; EXPECTED_BUF_BYTES];
        res_buf.splice(pos..pos + buffer.len(), buffer.into_iter().cloned());
        res_buf
    }
}
