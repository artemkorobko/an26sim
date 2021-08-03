use xplm::debugln;

use crate::{
    common::{bit::BitExt, chain::Mapper, percent::Percent},
    io::{
        index::*,
        params::{input::InputParams, ParamResult},
    },
};

use super::params::{XPlaneInputParams, XPlaneOutputParams};

macro_rules! oscale {
    ($value:expr, $src_min:expr, $src_max:expr, $dst_min:expr, $dst_max:expr) => {
        ($value.scale($src_min, $src_max, $dst_min, $dst_max) as u16).reverse_bits()
    };
}

#[derive(Default)]
pub struct SM2MXPlaneMapper;

impl SM2MXPlaneMapper {
    fn map(i: InputParams) -> ParamResult<XPlaneInputParams> {
        let mut o = XPlaneInputParams::default();
        let hi = (i.latitude_hi()?.reverse_bits() as i16 as f64).scale(0.0, 32768.0, 0.0, 90.0);
        let lo = (i.latitude_lo()?.reverse_bits() as f64).scale(0.0, 65535.0, 0.0, 0.0027465);
        o.location.latitude = hi + lo;
        let hi = (i.longitude_hi()?.reverse_bits() as i16 as f64).scale(0.0, 32768.0, 0.0, 360.0);
        let lo = (i.longitude_lo()?.reverse_bits() as f64).scale(0.0, 65535.0, 0.0, 0.010985);
        o.location.longitude = hi + lo;
        let val = i.altitude()?.reverse_bits() as f64;
        o.location.altitude = val;
        let val = (i.heading()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 359.99);
        o.orientation.heading = val;
        let val = (i.pitch()?.reverse_bits() as i16 as f32).scale(-32767.0, 32767.0, -45.0, 45.0);
        o.orientation.pitch = val;
        let val = (i.roll()?.reverse_bits() as i16 as f32).scale(-32767.0, 32767.0, -90.0, 90.0);
        o.orientation.roll = val;
        let val = (i.ailerons()?.reverse_bits() as i16 as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        o.surfaces.ailerons = val;
        let val = (i.elevator()?.reverse_bits() as i16 as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        o.surfaces.elevator = val;
        let val = (i.rudder()?.reverse_bits() as i16 as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
        o.surfaces.rudder = val;
        let val = (i.flaps()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        o.surfaces.flaps = val;
        let val = (i.engine_left()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 166.0);
        o.engines.left = val;
        let val = (i.engine_right()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 166.0);
        o.engines.right = val;
        let val = (i.gear_front()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        o.gears.front = val;
        let val = (i.gear_left()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        o.gears.left = val;
        let val = (i.gear_right()?.reverse_bits() as f32).scale(0.0, 32767.0, 0.0, 1.0);
        o.gears.right = val;
        let val = i.lights()?.reverse_bits();
        o.lights.landing = val.bit_test(0);
        o.lights.navigation = val.bit_test(1);
        o.lights.beacon = val.bit_test(2);
        o.reset = i.reset()?.reverse_bits().bit_test(0);
        Ok(o)
    }
}

impl Mapper<Option<InputParams>, Option<XPlaneInputParams>> for SM2MXPlaneMapper {
    fn map(&mut self, input: Option<InputParams>) -> Option<XPlaneInputParams> {
        if let Some(params) = input {
            let result = Self::map(params);
            match result {
                Ok(mapped) => Some(mapped),
                Err(error) => {
                    debugln!("{}", error.to_string());
                    None
                }
            }
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct XPlaneSM2MMapper;

impl Mapper<XPlaneOutputParams, Vec<u16>> for XPlaneSM2MMapper {
    fn map(&mut self, i: XPlaneOutputParams) -> Vec<u16> {
        let mut o = vec![0; 15];
        let val = i.terrain_distance.round() as u16;
        o[output::TERR_DIST_IDX] = val.reverse_bits();
        let val = i.location.altitude.round() as u16;
        o[output::ALT_IDX] = val.reverse_bits();
        let val = (i
            .orientation
            .heading
            .scale(0.0, 359.99, 0.0, 32767.0)
            .round()) as u16;
        o[output::HDG_IDX] = val.reverse_bits();
        let val = (i
            .orientation
            .pitch
            .scale(-45.0, 45.0, -32767.0, 32767.0)
            .round()) as u16;
        o[output::PITCH_IDX] = val.reverse_bits();
        o[output::ROLL_IDX] = oscale!(i.orientation.roll, -90.0, 90.0, -32767.0, 32767.0);
        o[output::AIL_IDX] = oscale!(i.surfaces.ailerons, -1.0, 1.0, -32767.0, 32767.0);
        o[output::ELEV_IDX] = oscale!(i.surfaces.elevator, -1.0, 1.0, -32767.0, 32767.0);
        o[output::RUD_IDX] = oscale!(i.surfaces.rudder, -1.0, 1.0, -32767.0, 32767.0);
        o[output::FLP_IDX] = oscale!(i.surfaces.flaps, 0.0, 1.0, 0.0, 32767.0);
        o[output::ENG_L_IDX] = oscale!(i.engines.left, 0.0, 166.0, 0.0, 32767.0);
        o[output::ENG_R_IDX] = oscale!(i.engines.right, 0.0, 166.0, 0.0, 32767.0);
        o[output::GEAR_F_IDX] = oscale!(i.gears.front, 0.0, 1.0, 0.0, 32767.0);
        o[output::GEAR_L_IDX] = oscale!(i.gears.left, 0.0, 1.0, 0.0, 32767.0);
        o[output::GEAR_R_IDX] = oscale!(i.gears.right, 0.0, 1.0, 0.0, 32767.0);
        let mut lights = 0;
        if i.lights.landing {
            lights.bit_set(0);
        }
        if i.lights.navigation {
            lights.bit_set(1);
        }
        if i.lights.beacon {
            lights.bit_set(2);
        }
        o[output::LIGHTS_IDX] = lights.reverse_bits();
        // o.push(lights);
        o
    }
}
