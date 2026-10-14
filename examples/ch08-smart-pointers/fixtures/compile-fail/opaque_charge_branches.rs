trait Charge {
    fn amount(&self) -> i32;
}

struct Order {
    unit_price: i32,
    count: i32,
}

struct Bundle {
    price: i32,
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

fn make_charge(as_order: bool) -> impl Charge {
    if as_order {
        Order {
            unit_price: 80,
            count: 1,
        }
    } else {
        Bundle { price: 120 }
    }
}

fn main() {
    println!("金額={}", make_charge(true).amount());
}
