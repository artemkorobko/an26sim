#[derive(Copy, Clone)]
pub enum Direction {
    Increment,
    Decrement,
}

impl Direction {
    pub fn reverse(self) -> Self {
        match self {
            Self::Increment => Self::Decrement,
            Self::Decrement => Self::Increment,
        }
    }
}
