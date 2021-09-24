use crate::{shared::pipeline::Mapper, xplane::output_params::XPlaneOutputParams};

#[derive(Default)]
pub struct XPlaneSM2MOutputMapper;

impl Mapper<XPlaneOutputParams, Vec<u8>> for XPlaneSM2MOutputMapper {
    fn map(&mut self, input: XPlaneOutputParams) -> Vec<u8> {
        (input.agl.round() as u16).to_be_bytes().to_vec()
    }
}
