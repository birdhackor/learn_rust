fn main() {
    let price: i32 = 80;
    let mut count: i32 = 2;
    let before = price * count;
    count += 1;
    let after = price * count;
    println!("原總額={before}，新總額={after}");
}
