use crate::{
    common::{percent::Percent, pipeline::Mapper},
    xplane::output_params::XPlaneOutputParams,
};

#[derive(Default)]
pub struct XPlaneSM2MOutputMapper;

impl XPlaneSM2MOutputMapper {
    pub fn terrain_distance(value: f32) -> u16 {
        value as u16
    }

    pub fn latitude(value: f64) -> u32 {
        value.scale(0.0, 90.0, 0.0, u32::MAX as f64) as u32
    }

    pub fn longitude(value: f64) -> u32 {
        value.scale(0.0, 360.0, 0.0, u32::MAX as f64) as u32
    }
}

impl Mapper<XPlaneOutputParams, Vec<u8>> for XPlaneSM2MOutputMapper {
    fn map(&mut self, input: XPlaneOutputParams) -> Vec<u8> {
        Self::terrain_distance(input.terrain_distance)
            .to_be_bytes()
            .to_vec()
    }
}
