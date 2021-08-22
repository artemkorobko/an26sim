pub fn bit_test(value: u16, index: u16) -> bool {
    (value >> index) & 0b1 == 1
}

pub fn bit_set(value: u16, index: u16) -> u16 {
    let mask = 1 << index;
    value | mask
}
