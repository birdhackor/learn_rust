pub mod readers;

use blake3::Hash;
use blake3::Hasher;
use std::io;
use std::io::Read;

// --8<-- [start:hash]
pub fn hash_reader<R: Read>(reader: &mut R) -> io::Result<Hash> {
    let mut hasher = Hasher::new();
    let mut buffer: [u8; 4096] = [0; 4096];
    loop {
        let count = match reader.read(&mut buffer[..]) {
            Ok(count) => count,
            Err(error) => {
                if error.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(error);
            }
        };
        if count == 0 {
            return Ok(hasher.finalize());
        }
        hasher.update(&buffer[..count]);
    }
}
// --8<-- [end:hash]

// --8<-- [start:fill]
fn fill_chunk<R: Read>(reader: &mut R, buffer: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;
    while filled < buffer.len() {
        match reader.read(&mut buffer[filled..]) {
            Ok(0) => break,
            Ok(count) => filled += count,
            Err(error) => {
                if error.kind() != io::ErrorKind::Interrupted {
                    return Err(error);
                }
            }
        }
    }
    Ok(filled)
}
// --8<-- [end:fill]

// --8<-- [start:equal]
#[derive(Debug)]
pub enum CompareReadError {
    Left(io::Error),
    Right(io::Error),
}

pub fn equal_readers<L: Read, R: Read>(
    left: &mut L,
    right: &mut R,
) -> Result<bool, CompareReadError> {
    let mut left_buffer: [u8; 4096] = [0; 4096];
    let mut right_buffer: [u8; 4096] = [0; 4096];
    loop {
        let left_count = match fill_chunk(left, &mut left_buffer[..]) {
            Ok(count) => count,
            Err(error) => return Err(CompareReadError::Left(error)),
        };
        let right_count = match fill_chunk(right, &mut right_buffer[..]) {
            Ok(count) => count,
            Err(error) => return Err(CompareReadError::Right(error)),
        };
        if left_buffer[..left_count] != right_buffer[..right_count] {
            return Ok(false);
        }
        if left_count == 0 {
            return Ok(true);
        }
    }
}
// --8<-- [end:equal]
