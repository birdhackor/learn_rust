use std::num::TryFromIntError;

// --8<-- [start:metrics]
#[derive(Debug)]
pub enum MetricError {
    Count(TryFromIntError),
    Overflow,
}

pub fn duplicate_bytes(len: u64, path_count: usize) -> Result<u64, MetricError> {
    if path_count < 2 {
        return Ok(0);
    }
    let extra_paths = match u64::try_from(path_count - 1) {
        Ok(count) => count,
        Err(error) => return Err(MetricError::Count(error)),
    };
    match len.checked_mul(extra_paths) {
        Some(bytes) => Ok(bytes),
        None => Err(MetricError::Overflow),
    }
}

pub fn add_duplicate_bytes(total: u64, bytes: u64) -> Result<u64, MetricError> {
    match total.checked_add(bytes) {
        Some(sum) => Ok(sum),
        None => Err(MetricError::Overflow),
    }
}
// --8<-- [end:metrics]
