struct Order {
    unit_price: i32,
    count: i32,
}

impl Order {
    fn into_order(&self) -> Self {
        *self
    }
}

fn main() {
    let order = Order {
        unit_price: 80,
        count: 2,
    };
    let next = order.into_order();
    println!("接手的小計={}", next.unit_price * next.count);
}
