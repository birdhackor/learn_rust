fn main() {
    let first = Box::new(120);
    let second = first;
    println!("接手值={}", *second);
    println!("原名稱={}", *first);
}
