mod size_support;

use crate::size_support::borrowed_bytes;
use crate::size_support::fixed_bytes;

fn main() {
    let values: [i32; 3] = [10, 20, 30];
    let part: &[i32] = &values[1..];
    let word: &str = "Rust!";
    let longer_word: &str = "Rust tutorial";

    println!("i32 型別的 bytes={}", fixed_bytes::<i32>());
    println!("三項 array 的 bytes={}", fixed_bytes::<[i32; 3]>());
    println!("slice reference 自身的 bytes={}", fixed_bytes::<&[i32]>());
    println!("借來的兩項資料 bytes={}", borrowed_bytes(part));
    println!("str reference 自身的 bytes={}", fixed_bytes::<&str>());
    println!("短文字的資料 bytes={}", borrowed_bytes(word));
    println!("長文字的資料 bytes={}", borrowed_bytes(longer_word));
    assert_eq!(borrowed_bytes(&values), 12);
    assert_eq!(borrowed_bytes(part), 8);
    assert_eq!(borrowed_bytes(word), 5);
    assert_eq!(borrowed_bytes(longer_word), 13);
}
