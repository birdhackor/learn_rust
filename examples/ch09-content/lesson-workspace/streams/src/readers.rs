use std::borrow::ToOwned;
use std::io;
use std::io::Read;

// --8<-- [start:short]
pub struct ShortReader {
    data: Vec<u8>,
    position: usize,
    max_per_read: usize,
}

pub fn short_reader(data: &[u8], max_per_read: usize) -> ShortReader {
    if max_per_read == 0 {
        panic!("測試來源的每次上限必須是正數");
    }
    ShortReader {
        data: data.to_owned(),
        position: 0,
        max_per_read,
    }
}

impl Read for ShortReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut copied = 0;
        while copied < buffer.len() && copied < self.max_per_read && self.position < self.data.len()
        {
            buffer[copied] = self.data.as_slice()[self.position];
            copied += 1;
            self.position += 1;
        }
        Ok(copied)
    }
}
// --8<-- [end:short]

// --8<-- [start:failure]
pub struct InterruptedThenFailure {
    step: usize,
}

pub fn failing_reader() -> InterruptedThenFailure {
    InterruptedThenFailure { step: 0 }
}

impl Read for InterruptedThenFailure {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let step = self.step;
        self.step += 1;
        if step == 0 {
            Err(io::Error::from(io::ErrorKind::Interrupted))
        } else if step == 1 {
            buffer[0] = 82;
            Ok(1)
        } else {
            Err(io::Error::from(io::ErrorKind::PermissionDenied))
        }
    }
}
// --8<-- [end:failure]
