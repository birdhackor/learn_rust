fn main() {
    let value = 80;
    let pointer: *const i32 = &raw const value;
    let read = *pointer;
    println!("讀取={read}");
}
