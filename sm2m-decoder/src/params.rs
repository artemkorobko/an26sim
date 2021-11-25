pub const PARAMS_COUNT: usize = 12;

#[derive(Default)]
pub struct SM2MParams {
    params: [u16; PARAMS_COUNT],
}

impl SM2MParams {
    pub fn set(&mut self, index: usize, value: u16) -> bool {
        if index < self.params.len() {
            self.params[index] = value;
            true
        } else {
            false
        }
    }

    pub fn get(&self, index: usize) -> Option<u16> {
        if index < self.params.len() {
            Some(self.params[index])
        } else {
            None
        }
    }
}
