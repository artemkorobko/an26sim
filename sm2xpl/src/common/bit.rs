pub trait BitExt: Sized {
    fn bit_test(&self, index: Self) -> bool;
    fn bit_set(&mut self, index: Self);
}

impl BitExt for u16 {
    fn bit_test(&self, index: Self) -> bool {
        (self >> index) & 0b1 == 1
    }

    fn bit_set(&mut self, index: Self) {
        let mask = (1 as Self) << index;
        *self |= mask;
    }
}
