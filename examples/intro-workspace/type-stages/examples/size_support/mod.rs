// --8<-- [start:fixed-bytes]
pub fn fixed_bytes<T>() -> usize {
    std::mem::size_of::<T>()
}
// --8<-- [end:fixed-bytes]

// --8<-- [start:borrowed-bytes]
pub fn borrowed_bytes<T: ?Sized>(value: &T) -> usize {
    std::mem::size_of_val(value)
}
// --8<-- [end:borrowed-bytes]

#[cfg(test)]
mod tests {
    use crate::size_support::borrowed_bytes;
    use crate::size_support::fixed_bytes;

    #[test]
    fn data_bytes_follow_the_borrowed_region() {
        let values: [i32; 3] = [10, 20, 30];
        assert_eq!(borrowed_bytes(&values), 3 * fixed_bytes::<i32>());
        assert_eq!(borrowed_bytes(&values[1..]), 2 * fixed_bytes::<i32>());
        assert_eq!(borrowed_bytes("Rust!"), 5);
        assert_eq!(borrowed_bytes("Rust tutorial"), 13);
        assert_eq!(borrowed_bytes(""), 0);
    }
}
