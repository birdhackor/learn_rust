fn array_bytes<T, const N: usize>(_: &[T; N]) -> usize {
    N * std::mem::size_of::<T>()
}

fn main() {
    let mut values: [i32; 2] = [80, 120];
    let switches: [bool; 3] = [true, false, true];
    let empty: [i32; 0] = [];
    let before = array_bytes(&values);
    values[0] = 200;
    let after = array_bytes(&values);
    assert_eq!(before, 8);
    assert_eq!(after, 8);
    assert_eq!(array_bytes(&switches), 3);
    assert_eq!(array_bytes(&empty), 0);
    println!("i32 array 修改前後的 bytes={before}、{after}");
    println!("bool array 的 bytes={}", array_bytes(&switches));
    println!("空 array 的 bytes={}", array_bytes(&empty));
}
