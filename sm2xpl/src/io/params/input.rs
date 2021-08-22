use super::generic::{InputParams, ParamsIOError, ParamsIOResult};

const LATITUDE_HI_IDX: usize = 0;
const LATITUDE_LO_IDX: usize = LATITUDE_HI_IDX + 1;
const LONGITUDE_HI_IDX: usize = LATITUDE_LO_IDX + 1;
const LONGITUDE_LO_IDX: usize = LONGITUDE_HI_IDX + 1;
const ALTITUDE_IDX: usize = LONGITUDE_LO_IDX + 1;
const HEADING_IDX: usize = ALTITUDE_IDX + 1;
const PITCH_IDX: usize = HEADING_IDX + 1;
const ROLL_IDX: usize = PITCH_IDX + 1;
const AILERONS_IDX: usize = ROLL_IDX + 1;
const ELEVATOR_IDX: usize = AILERONS_IDX + 1;
const RUDDER_IDX: usize = ELEVATOR_IDX + 1;
const FLAPS_IDX: usize = RUDDER_IDX + 1;
const ENGINE_L_IDX: usize = FLAPS_IDX + 1;
const ENGINE_R_IDX: usize = ENGINE_L_IDX + 1;
const GEAR_F_IDX: usize = ENGINE_R_IDX + 1;
const GEAR_L_IDX: usize = GEAR_F_IDX + 1;
const GEAR_R_IDX: usize = GEAR_L_IDX + 1;
const LIGHTS_IDX: usize = GEAR_R_IDX + 1;
const RESET_IDX: usize = LIGHTS_IDX + 1;
const PARAMS_COUNT: usize = RESET_IDX + 1;

impl InputParams for Vec<u16> {
    fn latitude_hi(&self) -> ParamsIOResult<u16> {
        self.read_param(LATITUDE_HI_IDX)
    }

    fn latitude_lo(&self) -> ParamsIOResult<u16> {
        self.read_param(LATITUDE_LO_IDX)
    }

    fn longitude_hi(&self) -> ParamsIOResult<u16> {
        self.read_param(LONGITUDE_HI_IDX)
    }

    fn longitude_lo(&self) -> ParamsIOResult<u16> {
        self.read_param(LONGITUDE_LO_IDX)
    }

    fn altitude(&self) -> ParamsIOResult<u16> {
        self.read_param(ALTITUDE_IDX)
    }

    fn heading(&self) -> ParamsIOResult<u16> {
        self.read_param(HEADING_IDX)
    }

    fn pitch(&self) -> ParamsIOResult<u16> {
        self.read_param(PITCH_IDX)
    }

    fn roll(&self) -> ParamsIOResult<u16> {
        self.read_param(ROLL_IDX)
    }

    fn ailerons(&self) -> ParamsIOResult<u16> {
        self.read_param(AILERONS_IDX)
    }

    fn elevator(&self) -> ParamsIOResult<u16> {
        self.read_param(ELEVATOR_IDX)
    }

    fn rudder(&self) -> ParamsIOResult<u16> {
        self.read_param(RUDDER_IDX)
    }

    fn flaps(&self) -> ParamsIOResult<u16> {
        self.read_param(FLAPS_IDX)
    }

    fn engine_left(&self) -> ParamsIOResult<u16> {
        self.read_param(ENGINE_L_IDX)
    }

    fn engine_right(&self) -> ParamsIOResult<u16> {
        self.read_param(ENGINE_R_IDX)
    }

    fn gear_front(&self) -> ParamsIOResult<u16> {
        self.read_param(GEAR_F_IDX)
    }

    fn gear_left(&self) -> ParamsIOResult<u16> {
        self.read_param(GEAR_L_IDX)
    }

    fn gear_right(&self) -> ParamsIOResult<u16> {
        self.read_param(GEAR_R_IDX)
    }

    fn lights(&self) -> ParamsIOResult<u16> {
        self.read_param(LIGHTS_IDX)
    }

    fn reset(&self) -> ParamsIOResult<u16> {
        self.read_param(RESET_IDX)
    }

    fn read_param(&self, idx: usize) -> ParamsIOResult<u16> {
        if self.len() > idx {
            Ok(self[idx])
        } else {
            Err(ParamsIOError::InvalidIndex(idx, self.len()))
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn should_read_params() {
        let vec = (0u16..PARAMS_COUNT as u16).collect::<Vec<u16>>();

        assert_eq!(vec.latitude_hi().unwrap(), 0);
        assert_eq!(vec.latitude_lo().unwrap(), 1);
        assert_eq!(vec.longitude_hi().unwrap(), 2);
        assert_eq!(vec.longitude_lo().unwrap(), 3);
        assert_eq!(vec.altitude().unwrap(), 4);
        assert_eq!(vec.heading().unwrap(), 5);
        assert_eq!(vec.pitch().unwrap(), 6);
        assert_eq!(vec.roll().unwrap(), 7);
        assert_eq!(vec.ailerons().unwrap(), 8);
        assert_eq!(vec.elevator().unwrap(), 9);
        assert_eq!(vec.rudder().unwrap(), 10);
        assert_eq!(vec.flaps().unwrap(), 11);
        assert_eq!(vec.engine_left().unwrap(), 12);
        assert_eq!(vec.engine_right().unwrap(), 13);
        assert_eq!(vec.gear_front().unwrap(), 14);
        assert_eq!(vec.gear_left().unwrap(), 15);
        assert_eq!(vec.gear_right().unwrap(), 16);
        assert_eq!(vec.lights().unwrap(), 17);
        assert_eq!(vec.reset().unwrap(), 18);
    }

    #[test]
    fn should_return_error_when_index_is_out_of_bounds() {
        let vec = vec![0u16; 10];

        let error = vec.lights().err().unwrap();

        assert_eq!(error.to_string(), "q23");
    }
}
