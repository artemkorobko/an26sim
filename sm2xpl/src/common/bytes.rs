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
