struct Order {
    unit_price: i32,
    count: i32,
}

impl Order {
    // --8<-- [start:read]
    fn subtotal(&self) -> i32 {
        self.unit_price * self.count
    }
    // --8<-- [end:read]

    // --8<-- [start:write]
    fn add_count(&mut self, additional: i32) {
        self.count += additional;
    }
    // --8<-- [end:write]

    // --8<-- [start:take]
    fn into_parts(self) -> (i32, i32) {
        (self.unit_price, self.count)
    }
    // --8<-- [end:take]

    // --8<-- [start:new]
    fn new(unit_price: i32, count: i32) -> Self {
        Self { unit_price, count }
    }
    // --8<-- [end:new]
}

fn main() {
    let mut order = Order::new(80, 2);
    let before = order.subtotal();
    order.add_count(1);
    let after = order.subtotal();
    println!("修改前={before}，修改後={after}");

    let (price, count) = order.into_parts();
    println!("接手後交出的單價={price}，數量={count}");
    assert_eq!(before, 160);
    assert_eq!(after, 240);
    assert_eq!(price, 80);
    assert_eq!(count, 3);
}
