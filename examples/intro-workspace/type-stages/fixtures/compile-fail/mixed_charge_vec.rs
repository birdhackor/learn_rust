struct Order {
    unit_price: i32,
    count: i32,
}

struct Bundle {
    price: i32,
}

trait Charge {
    fn amount(&self) -> i32;
}

impl Charge for Order {
    fn amount(&self) -> i32 {
        self.unit_price * self.count
    }
}

impl Charge for Bundle {
    fn amount(&self) -> i32 {
        self.price
    }
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
