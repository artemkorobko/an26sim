use super::direction::Direction;
use super::period::Period;

pub struct SequentialGenerator {
    pub min: u16,
    pub max: u16,
    pub step: u16,
    pub value: u16,
    pub period: Period,
    pub direction: Direction,
}

impl SequentialGenerator {
    pub fn generate(&mut self) {
        if self.should_generate() {
            self.period.reset();
            match self.direction {
                Direction::Increment => self.increment(),
                Direction::Decrement => self.decrement(),
            }
        }
    }

    fn should_generate(&mut self) -> bool {
        self.period.count();
        self.period.elapsed()
    }

    fn increment(&mut self) {
        self.value += self.step;
        if self.value >= self.max {
            self.value = self.max;
            self.direction = self.direction.reverse();
        }
    }

    fn decrement(&mut self) {
        self.value -= self.step;
        if self.value <= self.min {
            self.value = self.min;
            self.direction = self.direction.reverse();
        }
    }
}
