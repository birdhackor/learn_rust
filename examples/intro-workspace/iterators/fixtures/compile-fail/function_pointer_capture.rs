fn main() {
    let base = 10;
    let operation: fn(i32) -> i32 = |value| value + base;
    println!("{}", operation(3));
}
