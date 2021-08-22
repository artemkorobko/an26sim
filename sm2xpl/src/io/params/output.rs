use super::generic::{OutputParams, ParamsIOError, ParamsIOResult};

pub const TERRAIN_DISTANCE: usize = 0;

impl OutputParams for Vec<u16> {
    fn terrain_distance(mut self, value: u16) -> ParamsIOResult<Self> {
        self.write_param(TERRAIN_DISTANCE, value)?;
        Ok(self)
    }

    fn write_param(&mut self, idx: usize, value: u16) -> ParamsIOResult<()> {
        if self.len() > idx {
            self[idx] = value;
            Ok(())
        } else {
            Err(ParamsIOError::InvalidIndex(idx, self.len()))
        }
    }
}
