use std::{io, time::Duration};

pub trait Generator {
    fn write(&mut self, delta: Duration, buf: &mut dyn io::Write) -> io::Result<usize>;
    fn size_bytes(&self) -> usize;
}
