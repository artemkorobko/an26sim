use crate::{common::chain::Mapper, xplane::params::XPlaneOutputParams};

#[derive(Default)]
pub struct XPlaneSM2MOutputMapper;

impl Mapper<XPlaneOutputParams, Vec<u16>> for XPlaneSM2MOutputMapper {
    fn map(&mut self, input: XPlaneOutputParams) -> Vec<u16> {
        vec![input.terrain_distance.trunc() as u16]
    }
}
