use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParamsIOError {
    #[error("Parameter does not exists at index {0} in array of length {1}")]
    InvalidIndex(usize, usize),
}

pub type ParamsIOResult<T> = Result<T, ParamsIOError>;

pub trait InputParams {
    fn latitude_hi(&self) -> ParamsIOResult<u16>;
    fn latitude_lo(&self) -> ParamsIOResult<u16>;
    fn longitude_hi(&self) -> ParamsIOResult<u16>;
    fn longitude_lo(&self) -> ParamsIOResult<u16>;
    fn altitude(&self) -> ParamsIOResult<u16>;
    fn heading(&self) -> ParamsIOResult<u16>;
    fn pitch(&self) -> ParamsIOResult<u16>;
    fn roll(&self) -> ParamsIOResult<u16>;
    fn ailerons(&self) -> ParamsIOResult<u16>;
    fn elevator(&self) -> ParamsIOResult<u16>;
    fn rudder(&self) -> ParamsIOResult<u16>;
    fn flaps(&self) -> ParamsIOResult<u16>;
    fn engine_left(&self) -> ParamsIOResult<u16>;
    fn engine_right(&self) -> ParamsIOResult<u16>;
    fn gear_front(&self) -> ParamsIOResult<u16>;
    fn gear_left(&self) -> ParamsIOResult<u16>;
    fn gear_right(&self) -> ParamsIOResult<u16>;
    fn lights(&self) -> ParamsIOResult<u16>;
    fn reset(&self) -> ParamsIOResult<u16>;
    fn read_param(&self, idx: usize) -> ParamsIOResult<u16>;
}

pub trait OutputParams: Sized {
    fn terrain_distance(self, value: u16) -> ParamsIOResult<Self>;
    fn write_param(&mut self, idx: usize, value: u16) -> ParamsIOResult<()>;
}
