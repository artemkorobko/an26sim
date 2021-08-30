use crate::{
    common::{chain::Mapper, percent::Percent},
    io::params::generic::{InputParams, ParamsIOResult},
    xplane::input_params::XPlaneInputParams,
};

use super::bit::{bit_set, bit_test};

#[derive(Default)]
pub struct SM2MXPlaneInputMapper;

impl SM2MXPlaneInputMapper {
    fn map(input: Vec<u16>) -> ParamsIOResult<XPlaneInputParams> {
        let mut params = XPlaneInputParams::default();
        let mut reversed = input.latitude_hi()?.reverse_bits();
        let mut hi = (reversed as i16 as f64).scale(0.0, 32768.0, 0.0, 90.0);
        reversed = input.latitude_lo()?.reverse_bits();
        let mut lo = (reversed as f64).scale(0.0, 65535.0, 0.0, 0.0027465);
        params.location.latitude = hi + lo;
        reversed = input.longitude_hi()?.reverse_bits();
        hi = (reversed as i16 as f64).scale(0.0, 32768.0, 0.0, 360.0);
        reversed = input.longitude_lo()?.reverse_bits();
        lo = (reversed as f64).scale(0.0, 65535.0, 0.0, 0.010985);
        params.location.longitude = hi + lo;
        reversed = input.altitude()?.reverse_bits();
        params.location.altitude = reversed as f64;
        reversed = input.heading()?.reverse_bits();
        params.orientation.heading = (reversed as f32).scale(0.0, 32767.0, 0.0, 359.99);
        reversed = input.pitch()?.reverse_bits();
        params.orientation.pitch = (reversed as i16 as f32).scale(-32767.0, 32767.0, -45.0, 45.0);
        reversed = input.roll()?.reverse_bits();
        params.orientation.roll = (reversed as i16 as f32).scale(-32767.0, 32767.0, -90.0, 90.0);
        reversed = input.ailerons()?.reverse_bits();
        params.surfaces.ailerons = (reversed as i16 as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        reversed = input.elevator()?.reverse_bits();
        params.surfaces.elevator = (reversed as i16 as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        reversed = input.rudder()?.reverse_bits();
        params.surfaces.rudder = (reversed as i16 as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        reversed = input.flaps()?.reverse_bits();
        params.surfaces.flaps = (reversed as f32).scale(0.0, 32767.0, 0.0, 1.0);
        reversed = input.engine_left()?.reverse_bits();
        params.engines.left = (reversed as f32).scale(0.0, 32767.0, 0.0, 166.0);
        reversed = input.engine_right()?.reverse_bits();
        params.engines.right = (reversed as f32).scale(0.0, 32767.0, 0.0, 166.0);
        reversed = input.gear_front()?.reverse_bits();
        params.gears.front = (reversed as f32).scale(0.0, 32767.0, 0.0, 1.0);
        reversed = input.gear_left()?.reverse_bits();
        params.gears.left = (reversed as f32).scale(0.0, 32767.0, 0.0, 1.0);
        reversed = input.gear_right()?.reverse_bits();
        params.gears.right = (reversed as f32).scale(0.0, 32767.0, 0.0, 1.0);
        reversed = input.lights()?.reverse_bits();
        params.lights.landing = bit_test(reversed, 0);
        params.lights.navigation = bit_test(reversed, 1);
        params.lights.beacon = bit_test(reversed, 2);
        params.reset = bit_test(input.reset()?.reverse_bits(), 0);
        Ok(params)
    }
}

impl Mapper<Option<Vec<u16>>, Option<XPlaneInputParams>> for SM2MXPlaneInputMapper {
    fn map(&mut self, input: Option<Vec<u16>>) -> Option<XPlaneInputParams> {
        input.and_then(|params| {
            let result = Self::map(params);
            match result {
                Ok(mapped) => Some(mapped),
                Err(error) => {
                    xplm::debugln!("{}", error.to_string());
                    None
                }
            }
        })
    }
}

pub struct XPlaneSM2MInputMapper {
    pub latitude_hi: u16,
    pub latitude_lo: u16,
    pub longitude_hi: u16,
    pub longitude_lo: u16,
}

impl XPlaneSM2MInputMapper {
    pub fn new(latitude_hi: u16, latitude_lo: u16, longitude_hi: u16, longitude_lo: u16) -> Self {
        Self {
            latitude_hi,
            latitude_lo,
            longitude_hi,
            longitude_lo,
        }
    }
}

impl Default for XPlaneSM2MInputMapper {
    fn default() -> Self {
        Self {
            latitude_hi: 17606,
            latitude_lo: 21450,
            longitude_hi: 3193,
            longitude_lo: 15130,
        }
    }
}

impl Mapper<XPlaneInputParams, Vec<u16>> for XPlaneSM2MInputMapper {
    fn map(&mut self, input: XPlaneInputParams) -> Vec<u16> {
        let mut params = Vec::with_capacity(18);
        params.push(self.latitude_hi);
        params.push(self.latitude_hi);
        params.push(self.longitude_hi);
        params.push(self.longitude_lo);
        let mut param = input.location.altitude.round() as u16;
        params.push(param);
        param = input
            .orientation
            .heading
            .scale(0.0, 359.99, 0.0, 32767.0)
            .round() as u16;
        params.push(param);
        param = input
            .orientation
            .pitch
            .scale(-45.0, 45.0, -32767.0, 32767.0)
            .round() as u16;
        params.push(param);
        param = input
            .orientation
            .roll
            .scale(-90.0, 90.0, -32767.0, 32767.0)
            .round() as u16;
        params.push(param);
        param = input
            .surfaces
            .ailerons
            .scale(-1.0, 1.0, -32767.0, 32767.0)
            .round() as u16;
        params.push(param);
        param = input
            .surfaces
            .elevator
            .scale(-1.0, 1.0, -32767.0, 32767.0)
            .round() as u16;
        params.push(param);
        param = input
            .surfaces
            .rudder
            .scale(-1.0, 1.0, -32767.0, 32767.0)
            .round() as u16;
        params.push(param);
        param = input.surfaces.flaps.scale(0.0, 1.0, 0.0, 32767.0).round() as u16;
        params.push(param);
        param = input.engines.left.scale(0.0, 166.0, 0.0, 32767.0).round() as u16;
        params.push(param);
        param = input.engines.right.scale(0.0, 166.0, 0.0, 32767.0).round() as u16;
        params.push(param);
        param = input.gears.front.scale(0.0, 1.0, 0.0, 32767.0).round() as u16;
        params.push(param);
        param = input.gears.left.scale(0.0, 1.0, 0.0, 32767.0).round() as u16;
        params.push(param);
        param = input.gears.right.scale(0.0, 1.0, 0.0, 32767.0).round() as u16;
        params.push(param);
        let mut lights = 0;
        if input.lights.landing {
            lights = bit_set(lights, 0);
        }
        if input.lights.navigation {
            lights = bit_set(lights, 1);
        }
        if input.lights.beacon {
            lights = bit_set(lights, 2);
        }
        params.push(lights);
        params
    }
}
