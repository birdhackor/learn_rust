struct Order {
    unit_price: i32,
    count: i32,
}

fn main() {
    // --8<-- [start:tuple-array]
    let order = (120, 3, true);
    let (price, count, _) = order;
    println!("tuple 的小計={}", price * count);

    let prices = [120, 80, 200, 60, 40];
    let [first, .., last] = prices;
    println!("array 的首尾={first}、{last}");
    // --8<-- [end:tuple-array]

    // --8<-- [start:struct]
    let order = Order {
        unit_price: 80,
        count: 2,
    };
    let Order {
        unit_price: price,
        count,
    } = order;
    println!("struct 的單價={price}，數量={count}");
    // --8<-- [end:struct]

    // --8<-- [start:rest]
    let order = Order {
        unit_price: 200,
        count: 1,
    };
    let Order { unit_price, .. } = order;
    println!("只取單價={unit_price}");
    // --8<-- [end:rest]
}
