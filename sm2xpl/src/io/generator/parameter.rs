use bytes::BufMut;
use rand::Rng;

use std::{io, mem};

pub trait Parameter: Sized {
    fn increase(&self, target: Self) -> Option<Self>;
    fn decrease(&self, target: Self) -> Option<Self>;
    fn size_bytes(&self) -> usize;
    fn to_be_bytes_vec(&self) -> Vec<u8>;
    fn random() -> Self;
    fn zero() -> Self;
    fn write(&self, buf: &mut dyn io::Write) -> io::Result<usize>;
}

macro_rules! impl_parameter {
    ($type:ty) => {
        impl Parameter for $type {
            fn increase(&self, target: Self) -> Option<Self> {
                let (value, overflow) = self.overflowing_add(target);
                match overflow {
                    true => None,
                    false => Some(value),
                }
            }

            fn decrease(&self, target: Self) -> Option<Self> {
                let (value, overflow) = self.overflowing_sub(target);
                match overflow {
                    true => None,
                    false => Some(value),
                }
            }

            fn size_bytes(&self) -> usize {
                mem::size_of::<$type>()
            }

            fn to_be_bytes_vec(&self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }

            fn random() -> Self {
                rand::thread_rng().gen_range(<$type>::MIN..<$type>::MAX)
            }

            fn zero() -> Self {
                0
            }

            fn write(&self, buf: &mut dyn io::Write) -> io::Result<usize> {
                buf.write(&self.to_be_bytes())
            }
        }
    };
}

impl_parameter!(u16);
impl_parameter!(i16);
impl_parameter!(u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increase_u16() {
        let result = 10u16.increase(5).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn decrease_u16() {
        let result = 10u16.decrease(5).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn overflow_max_u16() {
        let result = u16::MAX.increase(1);
        assert!(result.is_none());
    }

    #[test]
    fn overflow_min_u16() {
        let result = u16::MIN.decrease(1);
        assert!(result.is_none());
    }

    #[test]
    fn return_u16_size_bytes() {
        let result = 0u16.size_bytes();
        assert_eq!(result, 2);
    }

    #[test]
    fn return_u16_bytes_vec() {
        let result = 513u16.to_be_bytes_vec();
        assert_eq!(result, vec![2, 1]);
    }

    #[test]
    fn return_u16_random() {
        assert_ne!(u16::random(), u16::random());
    }

    #[test]
    fn return_u16_zero() {
        assert_eq!(u16::zero(), 0);
    }

    #[test]
    fn increase_i16() {
        let result = 10i16.increase(5).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn decrease_i16() {
        let result = 10i16.decrease(5).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn overflow_max_i16() {
        let result = i16::MAX.increase(1);
        assert!(result.is_none());
    }

    #[test]
    fn overflow_min_i16() {
        let result = i16::MIN.decrease(1);
        assert!(result.is_none());
    }

    #[test]
    fn return_i16_size_bytes() {
        let result = 0i16.size_bytes();
        assert_eq!(result, 2);
    }

    #[test]
    fn return_i16_bytes_vec() {
        let result = (-1i16).to_be_bytes_vec();
        assert_eq!(result, vec![255, 255]);
    }

    #[test]
    fn return_i16_random() {
        assert_ne!(i16::random(), i16::random());
    }

    #[test]
    fn return_i16_zero() {
        assert_eq!(i16::zero(), 0);
    }

    #[test]
    fn increase_u32() {
        let result = 10u32.increase(5).unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn decrease_u32() {
        let result = 10u32.decrease(5).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn overflow_max_u32() {
        let result = u32::MAX.increase(1);
        assert!(result.is_none());
    }

    #[test]
    fn overflow_min_u32() {
        let result = u32::MIN.decrease(1);
        assert!(result.is_none());
    }

    #[test]
    fn return_u32_size_bytes() {
        let result = 0u32.size_bytes();
        assert_eq!(result, 4);
    }

    #[test]
    fn return_u32_bytes_vec() {
        let result = 67305985u32.to_be_bytes_vec();
        assert_eq!(result, vec![4, 3, 2, 1]);
    }

    #[test]
    fn return_u32_random() {
        assert_ne!(u32::random(), u32::random());
    }

    #[test]
    fn return_u32_zero() {
        assert_eq!(u32::zero(), 0);
    }
}
