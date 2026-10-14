// --8<-- [start:sizes]
pub fn fixed_bytes<T>() -> usize {
    std::mem::size_of::<T>()
}

pub fn borrowed_bytes<T: ?Sized>(value: &T) -> usize {
    std::mem::size_of_val(value)
}
// --8<-- [end:sizes]

#[cfg(test)]
mod tests {
    use crate::sizes::borrowed_bytes;
    use crate::sizes::fixed_bytes;

    #[test]
    fn data_bytes_follow_the_borrowed_region() {
        let values: [i32; 3] = [10, 20, 30];
        assert_eq!(borrowed_bytes(&values), 3 * fixed_bytes::<i32>());
        assert_eq!(borrowed_bytes(&values[1..]), 2 * fixed_bytes::<i32>());
        assert_eq!(borrowed_bytes("Rust!"), 5);
        assert_eq!(borrowed_bytes(""), 0);
    }
}
