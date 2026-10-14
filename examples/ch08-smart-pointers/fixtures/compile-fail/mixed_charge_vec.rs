struct Order {
    unit_price: i32,
    count: i32,
}

struct Bundle {
    price: i32,
}

fn main() {
    let values = vec![
        Order {
            unit_price: 80,
            count: 1,
        },
        Bundle { price: 120 },
    ];
    println!("項目數={}", values.len());
}
