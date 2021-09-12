pub fn bit_test(value: i16, index: i16) -> bool {
    (value >> index) & 0b1 == 1
}

pub fn bit_set(value: i16, index: i16) -> i16 {
    let mask = 1 << index;
    value | mask
}
