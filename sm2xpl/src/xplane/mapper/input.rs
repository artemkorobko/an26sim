use crate::{
    common::{chain::Mapper, percent::Percent},
    io::input_params::{InputParamType, InputParameter},
    xplane::{input_params::XPlaneInputParams, mapper::bit::bit_test},
};

pub struct SM2MXPlaneInputMapper {
    default: XPlaneInputParams,
}

impl SM2MXPlaneInputMapper {
    pub fn new(default: XPlaneInputParams) -> Self {
        Self { default }
    }

    fn map_params(&self, input: Vec<InputParameter>) -> XPlaneInputParams {
        let mut params = self.default;
        for param in input {
            match param.ip_type {
                InputParamType::LatitudeHi => {
                    params.latitude += (param.value as f64).scale(0.0, 32768.0, 0.0, 90.0)
                }
                InputParamType::LatitudeLo => {
                    params.latitude += (param.value as f64).scale(0.0, 65535.0, 0.0, 0.0027465);
                }
                InputParamType::LongitudeHi => {
                    params.longitude += (param.value as f64).scale(0.0, 32768.0, 0.0, 360.0);
                }
                InputParamType::LongitudeLo => {
                    params.longitude += (param.value as f64).scale(0.0, 65535.0, 0.0, 0.010985);
                }
                InputParamType::Altitude => {
                    params.altitude = param.value as f64;
                }
                InputParamType::Heading => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 359.99);
                }
                InputParamType::Pitch => {
                    params.heading = (param.value as f32).scale(-32767.0, 32767.0, -45.0, 45.0);
                }
                InputParamType::Roll => {
                    params.heading = (param.value as f32).scale(-32767.0, 32767.0, -90.0, 90.0);
                }
                InputParamType::Ailerons => {
                    params.heading = (param.value as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
                }
                InputParamType::Elevator => {
                    params.heading = (param.value as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
                }
                InputParamType::Rudder => {
                    params.heading = (param.value as f32).scale(-32767.0, 32767.0, -1.0, 1.0);
                }
                InputParamType::Flaps => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 1.0);
                }
                InputParamType::EngineLeft => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 166.0);
                }
                InputParamType::EngineRight => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 166.0);
                }
                InputParamType::GearFront => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 1.0);
                }
                InputParamType::GearLeft => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 1.0);
                }
                InputParamType::GearRight => {
                    params.heading = (param.value as f32).scale(0.0, 32767.0, 0.0, 1.0);
                }
                InputParamType::Lights => {
                    params.light_landing = bit_test(param.value, 0);
                    params.light_navigation = bit_test(param.value, 1);
                    params.light_beacon = bit_test(param.value, 2);
                }
                InputParamType::Reset => {
                    params.reset = bit_test(param.value, 0);
                }
            }
        }
        params
    }
}

impl Mapper<Option<Vec<InputParameter>>, Option<XPlaneInputParams>> for SM2MXPlaneInputMapper {
    fn map(&mut self, input: Option<Vec<InputParameter>>) -> Option<XPlaneInputParams> {
        input.and_then(|params| Some(self.map_params(params)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_map_default_params() {
        let default = Default::default();
        let mut mapper = SM2MXPlaneInputMapper::new(default);

        let params = mapper.map(Some(Vec::new())).unwrap();

        assert_eq!(params, default);
    }

    #[test]
    fn should_return_none_when_input_params_are_absent() {
        let mut mapper = default_mapper();

        let params = mapper.map(None);

        assert!(params.is_none());
    }

    #[test]
    fn should_map_latitude() {}

    #[test]
    fn should_map_longitude() {}

    #[test]
    fn should_map_altitude() {
        let mut mapper = default_mapper();
        let input = input_with_param(InputParamType::Altitude, 100);

        let params = mapper.map(input).unwrap();

        assert_eq!(params.altitude, 100.0);
    }

    #[test]
    fn should_map_heading() {
        // println!("{}", 250.0f32.scale(0.0, 359.99, 0.0, 32767.0));
        let mut mapper = default_mapper();
        let input = input_with_param(InputParamType::Heading, 22755);

        let params = mapper.map(input).unwrap();

        assert_eq!(params.heading, 250.0);
    }

    fn default_mapper() -> SM2MXPlaneInputMapper {
        SM2MXPlaneInputMapper::new(Default::default())
    }

    fn input_with_param(ip_type: InputParamType, value: i16) -> Option<Vec<InputParameter>> {
        Some(vec![InputParameter { ip_type, value }])
    }
}
