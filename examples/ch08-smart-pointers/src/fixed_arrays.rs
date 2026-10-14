// --8<-- [start:fixed-len]
pub fn fixed_len<T, const N: usize>(_: &[T; N]) -> usize {
    N
}
// --8<-- [end:fixed-len]
