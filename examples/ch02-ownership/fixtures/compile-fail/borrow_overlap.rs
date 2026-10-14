fn main() {
    let mut count: i32 = 3;
    let reference = &count;
    count += 1;
    println!("借用值={}", *reference);
}
