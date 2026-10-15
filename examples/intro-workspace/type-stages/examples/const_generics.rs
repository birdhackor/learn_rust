mod array_support;

use crate::array_support::fixed_len;

fn main() {
    let counts: [i32; 3] = [10, 20, 30];
    let switches: [bool; 2] = [true, false];
    println!("由 i32 array 推導 N={}", fixed_len(&counts));
    println!("由 bool array 推導 N={}", fixed_len(&switches));
    println!("明選 i32 與常數 3，得到 N={}", fixed_len::<i32, 3>(&counts));
    println!("只借 array，原資料仍可讀={}", counts[0]);
    assert_eq!(fixed_len(&counts), 3);
    assert_eq!(fixed_len(&switches), 2);
}
