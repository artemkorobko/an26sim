pub const MAX_PARAMS_COUNT: usize = 30;

pub struct Params {
    pub buf: [u16; MAX_PARAMS_COUNT],
    pub index: usize,
    pub count: usize,
}

impl Params {
    pub fn new(count: usize) -> Self {
        Self {
            buf: [0; MAX_PARAMS_COUNT],
            index: 0,
            count,
        }
    }

    pub fn register(&mut self, param: u16) -> bool {
        if self.has_capacity() {
            self.buf[self.index] = param;
            self.index += 1;
            self.has_capacity()
        } else {
            false
        }
    }

    fn has_capacity(&self) -> bool {
        self.index < self.count && self.index <= MAX_PARAMS_COUNT
    }
}

pub enum SM2MParamsState {
    DetectMarker,
    DetectParamsCount(usize),
    WaitForMarker(usize),
    ReadParams(Params),
}
