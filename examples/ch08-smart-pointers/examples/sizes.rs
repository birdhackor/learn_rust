use ch08_smart_pointers::sizes::borrowed_bytes;
use ch08_smart_pointers::sizes::fixed_bytes;

fn main() {
    let values: [i32; 3] = [10, 20, 30];
    let part = &values[1..];
    let word: &str = "Rust!";
    println!("i32型別的bytes={}", fixed_bytes::<i32>());
    println!("三項array的bytes={}", fixed_bytes::<[i32; 3]>());
    println!("slice reference自身的bytes={}", fixed_bytes::<&[i32]>());
    println!("借來的兩項資料bytes={}", borrowed_bytes(part));
    println!("str reference自身的bytes={}", fixed_bytes::<&str>());
    println!("借來的文字bytes={}", borrowed_bytes(word));
}
