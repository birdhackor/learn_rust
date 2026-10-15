struct Order {
    unit_price: i32,
    count: i32,
}

fn subtotal((price, count): (i32, i32)) -> i32 {
    price * count
}

fn main() {
    let order = (80, 2, false);
    let (price, count, _) = order;
    assert_eq!(subtotal((price, count)), 160);

    let prices = [80, 120, 200, 60];
    let [first, .., last] = prices;
    assert_eq!(first + last, 140);

    let order = Order {
        unit_price: 80,
        count: 2,
    };
    let Order {
        unit_price: price,
        count,
    } = order;
    assert_eq!(price * count, 160);
    println!("tuple 與 struct 小計都是160，array 首尾合計140");
}
