fn main() {
    let first = intro_modules_basic::pricing::total_with_shipping(80, 1);
    let second = intro_modules_basic::pricing::total_with_shipping(120, 1);
    println!("兩筆訂單總額={}", first + second);
}
