use crate::{shared::pipeline::Mapper, xplane::input_params::XPlaneInputParams};

#[derive(Default)]
pub struct SM2MXPlaneInputMapper;

impl Mapper<Option<Vec<u8>>, Option<XPlaneInputParams>> for SM2MXPlaneInputMapper {
    fn map(&mut self, input: Option<Vec<u8>>) -> Option<XPlaneInputParams> {
        input
            .filter(|params| params.len() == XPlaneInputParams::expected_buf_bytes())
            .map(|params| params.as_slice().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_none_when_input_params_are_absent() {
        let mut mapper = SM2MXPlaneInputMapper::default();

        let params = mapper.map(None);

        assert!(params.is_none());
    }

    #[test]
    fn return_none_when_buffer_is_too_small() {
        let mut mapper = SM2MXPlaneInputMapper::default();

        let params = mapper.map(Some(Vec::new()));

        assert!(params.is_none());
    }
}
