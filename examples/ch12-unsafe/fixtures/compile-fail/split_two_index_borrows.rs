fn split_with_indices(
    values: &mut [i32],
    mid: usize,
) -> Option<(&mut [i32], &mut [i32])> {
    if mid > values.len() {
        return None;
    }
    Some((&mut values[..mid], &mut values[mid..]))
}

fn main() {}
