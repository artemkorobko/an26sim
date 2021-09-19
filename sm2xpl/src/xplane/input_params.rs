use std::cell::Ref;

use bytes::Buf;

use crate::common::percent::Percent;

use super::{dataref::collection::DataRefs, mapper::transcoder::*};

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
        params.latitude = latitude::decode(buffer.get_u32());
        params.longitude = longitude::decode(buffer.get_u32());
        params.altitude = altitude::decode(buffer.get_i16());
        params.heading = heading::decode(buffer.get_u16());
        params.pitch = pitch::decode(buffer.get_i16());
        params.roll = roll::decode(buffer.get_i16());
        params.ailerons = ailerons::decode(buffer.get_i16());
        params.elevator = elevator::decode(buffer.get_i16());
        params.rudder = rudder::decode(buffer.get_i16());
        params.flaps = flaps::decode(buffer.get_u16());
        params.engine_left = engine::decode(buffer.get_u16());
        params.engine_right = engine::decode(buffer.get_u16());
        params.gear_front = gear::decode(buffer.get_u16());
        params.gear_left = gear::decode(buffer.get_u16());
        params.gear_right = gear::decode(buffer.get_u16());
        let (landing, navigation, beacon) = light::decode(buffer.get_u16());
        params.light_landing = landing;
        params.light_navigation = navigation;
        params.light_beacon = beacon;
        params.reset = reset::decode(buffer.get_u16());
        params
    }
}

#[cfg(test)]
mod tests {
    use bytes::BufMut;
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn read_from_input_buffer() {
        let mut buffer = Vec::with_capacity(EXPECTED_BUF_BYTES);
        buffer.put_u32(2402881062); // latitude
        buffer.put_u32(368431135); // longitude
        buffer.put_i16(150); // altitude
        buffer.put_i16(16384); // heading
        buffer.put_i16(-32767); // pitch
        buffer.put_i16(-32767); // roll
        buffer.put_i16(-32767); // ailerons
        buffer.put_i16(32767); // elevator
        buffer.put_i16(0); // rudder
        buffer.put_u16(32768); // flaps
        buffer.put_u16(0); // engine left
        buffer.put_u16(32768); // engine right
        buffer.put_u16(0); // gear front
        buffer.put_u16(32768); // gear left
        buffer.put_u16(65535); // gear right
        buffer.put_u16(7); // light
        buffer.put_u16(1); // reset

        let params = XPlaneInputParams::from(buffer.as_ref());

        assert_float_eq!(params.latitude, 50.351791, abs <= 0.00001);
        assert_float_eq!(params.longitude, 30.881541, abs <= 0.00001);
        assert_float_eq!(params.altitude, 150.0, abs <= 0.001);
        assert_float_eq!(params.heading, 90.0, abs <= 0.01);
        assert_float_eq!(params.pitch, -45.0, abs <= 0.01);
        assert_float_eq!(params.roll, -90.0, abs <= 0.01);
        assert_float_eq!(params.ailerons, -1.0, abs <= 0.001);
        assert_float_eq!(params.elevator, 1.0, abs <= 0.001);
        assert_float_eq!(params.rudder, 0.0, abs <= 0.001);
        assert_float_eq!(params.flaps, 0.5, abs <= 0.001);
        assert_float_eq!(params.engine_left, 0.0, abs <= 0.01);
        assert_float_eq!(params.engine_right, 83.0, abs <= 0.01);
        assert_float_eq!(params.gear_front, 0.0, abs <= 0.01);
        assert_float_eq!(params.gear_left, 0.5, abs <= 0.01);
        assert_float_eq!(params.gear_right, 1.0, abs <= 0.01);
        assert_eq!(params.light_landing, true);
        assert_eq!(params.light_navigation, true);
        assert_eq!(params.light_landing, true);
        assert_eq!(params.reset, true);
    }
}
