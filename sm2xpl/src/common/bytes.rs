use std::mem;

use bytes::{Buf, Bytes};

pub trait BytesExt {
    fn try_get_u32(&mut self) -> Option<u32>;
    fn try_get_u16(&mut self) -> Option<u16>;
    fn try_get_i16(&mut self) -> Option<i16>;
}

impl BytesExt for Bytes {
    fn try_get_u32(&mut self) -> Option<u32> {
        if self.remaining() >= mem::size_of::<u32>() {
            Some(self.get_u32())
        } else {
            None
        }
    }

    fn try_get_u16(&mut self) -> Option<u16> {
        if self.remaining() >= mem::size_of::<u16>() {
            Some(self.get_u16())
        } else {
            None
        }
    }

    fn try_get_i16(&mut self) -> Option<i16> {
        if self.remaining() >= mem::size_of::<i16>() {
            Some(self.get_i16())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_u32() {
        let mut bytes = Bytes::from(vec![1u8, 2u8, 3u8, 4u8]);

        let value = bytes.try_get_u32().unwrap();

        assert_eq!(value, 16909060);
    }

    #[test]
    fn dont_return_u32() {
        let mut bytes = Bytes::from(vec![]);

        let value = bytes.try_get_u32();

        assert!(value.is_none());
    }

    #[test]
    fn return_u16() {
        let mut bytes = Bytes::from(vec![1u8, 2u8]);

        let value = bytes.try_get_u16().unwrap();

        assert_eq!(value, 258);
    }

    #[test]
    fn dont_return_u16() {
        let mut bytes = Bytes::from(vec![]);

        let value = bytes.try_get_u16();

        assert!(value.is_none());
    }

    #[test]
    fn return_i16() {
        let mut bytes = Bytes::from(vec![255u8, 5u8]);

        let value = bytes.try_get_i16().unwrap();

        assert_eq!(value, -251);
    }

    #[test]
    fn dont_return_i16() {
        let mut bytes = Bytes::from(vec![]);

        let value = bytes.try_get_i16();

        assert!(value.is_none());
    }
}
