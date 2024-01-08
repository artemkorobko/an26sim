pub struct Period {
    limit: usize,
    count: usize,
}

impl Period {
    pub fn new(limit: usize) -> Self {
        Self { limit, count: 0 }
    }

    pub fn count(&mut self) {
        self.count += 1;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn elapsed(&mut self) -> bool {
        self.count >= self.limit
    }
}
