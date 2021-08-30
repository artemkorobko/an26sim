use super::generic::{OutputParams, ParamsIOError, ParamsIOResult};

const TERRAIN_DISTANCE: usize = 0;
const PARAMS_COUNT: usize = TERRAIN_DISTANCE + 1;

impl OutputParams for Vec<u16> {
    fn new_output_params() -> Self {
        (0u16..PARAMS_COUNT as u16).collect()
    }

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

mod test {
    use super::*;

    #[test]
    fn should_write_params() {
        let distance = 1;

        let vec = Vec::new_output_params().terrain_distance(distance).unwrap();

        assert_eq!(vec, vec![distance]);
    }

    #[test]
    fn should_return_error_when_index_is_out_of_bounds() {
        let error = Vec::new().terrain_distance(1).err().unwrap();

        assert_eq!(
            error.to_string(),
            "Parameter does not exists at index 0 in array of length 0"
        );
    }
}
