const PARAMS_COUNT: usize = 12;

pub struct SM2MParams {
    params: [u16; PARAMS_COUNT],
}

impl Default for SM2MParams {
    fn default() -> Self {
        Self {
            params: [0u16; PARAMS_COUNT],
        }
    }
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
