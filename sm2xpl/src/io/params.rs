use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParamError {
    #[error("Parameter does not exists at index {0} in array of length {1}")]
    InvalidIndex(usize, usize),
    #[error("Input parameters vec has {0} params, expected at least {1}")]
    InvalidInput(usize, usize),
}

pub type ParamResult<T> = Result<T, ParamError>;

pub mod input {
    pub const LAT_HI_IDX: usize = 0;
    pub const LAT_LO_IDX: usize = LAT_HI_IDX + 1;
    pub const LON_HI_IDX: usize = LAT_LO_IDX + 1;
    pub const LON_LO_IDX: usize = LON_HI_IDX + 1;
    pub const ALT_IDX: usize = LON_LO_IDX + 1;
    pub const HDG_IDX: usize = ALT_IDX + 1;
    pub const PITCH_IDX: usize = HDG_IDX + 1;
    pub const ROLL_IDX: usize = PITCH_IDX + 1;
    pub const AIL_IDX: usize = ROLL_IDX + 1;
    pub const ELEV_IDX: usize = AIL_IDX + 1;
    pub const RUD_IDX: usize = ELEV_IDX + 1;
    pub const FLP_IDX: usize = RUD_IDX + 1;
    pub const ENG_L_IDX: usize = FLP_IDX + 1;
    pub const ENG_R_IDX: usize = ENG_L_IDX + 1;
    pub const GEAR_F_IDX: usize = ENG_R_IDX + 1;
    pub const GEAR_L_IDX: usize = GEAR_F_IDX + 1;
    pub const GEAR_R_IDX: usize = GEAR_L_IDX + 1;
    pub const LIGHTS_IDX: usize = GEAR_R_IDX + 1;
    pub const RESET_IDX: usize = LIGHTS_IDX + 1;
    pub const PARAMS_COUNT: usize = RESET_IDX;

    pub struct InputParams {
        params: Vec<u16>,
    }

    impl InputParams {
        pub fn from_vec(vec: Vec<u16>) -> super::ParamResult<InputParams> {
            if vec.len() >= PARAMS_COUNT {
                Ok(InputParams { params: vec })
            } else {
                Err(super::ParamError::InvalidInput(vec.len(), PARAMS_COUNT))
            }
        }

        pub fn latitude_hi(&self) -> super::ParamResult<u16> {
            self.read_param(LAT_HI_IDX)
        }

        pub fn latitude_lo(&self) -> super::ParamResult<u16> {
            self.read_param(LAT_LO_IDX)
        }

        pub fn longitude_hi(&self) -> super::ParamResult<u16> {
            self.read_param(LON_HI_IDX)
        }

        pub fn longitude_lo(&self) -> super::ParamResult<u16> {
            self.read_param(LON_LO_IDX)
        }

        pub fn altitude(&self) -> super::ParamResult<u16> {
            self.read_param(ALT_IDX)
        }

        pub fn heading(&self) -> super::ParamResult<u16> {
            self.read_param(HDG_IDX)
        }

        pub fn pitch(&self) -> super::ParamResult<u16> {
            self.read_param(PITCH_IDX)
        }

        pub fn roll(&self) -> super::ParamResult<u16> {
            self.read_param(ROLL_IDX)
        }

        pub fn ailerons(&self) -> super::ParamResult<u16> {
            self.read_param(AIL_IDX)
        }

        pub fn elevator(&self) -> super::ParamResult<u16> {
            self.read_param(ELEV_IDX)
        }

        pub fn rudder(&self) -> super::ParamResult<u16> {
            self.read_param(RUD_IDX)
        }

        pub fn flaps(&self) -> super::ParamResult<u16> {
            self.read_param(FLP_IDX)
        }

        pub fn engine_left(&self) -> super::ParamResult<u16> {
            self.read_param(ENG_L_IDX)
        }

        pub fn engine_right(&self) -> super::ParamResult<u16> {
            self.read_param(ENG_R_IDX)
        }

        pub fn gear_front(&self) -> super::ParamResult<u16> {
            self.read_param(GEAR_F_IDX)
        }

        pub fn gear_left(&self) -> super::ParamResult<u16> {
            self.read_param(GEAR_L_IDX)
        }

        pub fn gear_right(&self) -> super::ParamResult<u16> {
            self.read_param(GEAR_R_IDX)
        }

        pub fn lights(&self) -> super::ParamResult<u16> {
            self.read_param(LIGHTS_IDX)
        }

        pub fn reset(&self) -> super::ParamResult<u16> {
            self.read_param(RESET_IDX)
        }

        fn read_param(&self, idx: usize) -> super::ParamResult<u16> {
            if self.params.len() > idx {
                Ok(self.params[idx])
            } else {
                Err(super::ParamError::InvalidIndex(idx, self.params.len()))
            }
        }
    }
}
