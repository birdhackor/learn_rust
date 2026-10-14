use ch01_basics::pricing::total_with_shipping;

fn main() {
    let first = total_with_shipping(80, 1);
    let second = total_with_shipping(120, 1);
    let total = first + second;
    println!("兩筆訂單總額={total}");
}
