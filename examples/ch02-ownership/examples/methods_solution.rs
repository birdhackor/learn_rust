struct Order {
    unit_price: i32,
    count: i32,
}

impl Order {
    fn new(unit_price: i32, count: i32) -> Self {
        Self { unit_price, count }
    }

    fn subtotal(&self) -> i32 {
        self.unit_price * self.count
    }

    fn reset_count(&mut self) {
        self.count = 0;
    }
}

fn main() {
    let mut order = Order::new(80, 3);
    assert_eq!(order.subtotal(), 240);
    order.reset_count();
    assert_eq!(order.count, 0);
    assert_eq!(order.unit_price, 80);
    assert_eq!(order.subtotal(), 0);
    println!(
        "清空後數量={}，單價={}，小計=0",
        order.count, order.unit_price
    );
}
