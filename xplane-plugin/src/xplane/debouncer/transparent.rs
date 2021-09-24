use super::generic::Debouncer;

pub struct TransparentDebouncer<T> {
    value: T,
}

impl<T: Default> TransparentDebouncer<T> {
    pub fn new() -> Self {
        Self {
            value: T::default(),
        }
    }
}

impl<T: Copy> Debouncer<T> for TransparentDebouncer<T> {
    fn debounce(&mut self, target: T, _: &std::time::Duration) -> T {
        target
    }

    fn integrate(&mut self, _: &std::time::Duration) -> T {
        self.value
    }

    fn assign(&mut self, target: T) -> T {
        self.value = target;
        self.value
    }
}
