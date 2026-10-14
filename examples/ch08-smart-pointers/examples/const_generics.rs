use ch08_smart_pointers::fixed_arrays::fixed_len;

fn main() {
    let counts: [i32; 3] = [10, 20, 30];
    let switches: [bool; 2] = [true, false];
    println!("由i32 array推導N={}", fixed_len(&counts));
    println!("由bool array推導N={}", fixed_len(&switches));
    println!("明選型別i32與常數3，得到N={}", fixed_len::<i32, 3>(&counts));
    println!("只借array，原資料仍可讀={}", counts[0]);
}
