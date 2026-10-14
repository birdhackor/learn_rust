struct Order {
    unit_price: i32,
    count: i32,
}

fn main() {
    let first = Order {
        unit_price: 120,
        count: 3,
    };
    let second = first;
    println!("新訂單小計={}", second.unit_price * second.count);
    println!("舊名稱的數量={}", first.count);
}
