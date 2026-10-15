// --8<-- [start:parameter]
fn subtotal((price, count): (i32, i32)) -> i32 {
    price * count
}
// --8<-- [end:parameter]

fn main() {
    println!("參數解構的小計={}", subtotal((120, 3)));

    // --8<-- [start:for]
    let orders = [(120, 3), (80, 2)];
    let mut total = 0;
    for (price, count) in orders {
        total += price * count;
    }
    println!("兩筆小計合計={total}");
    // --8<-- [end:for]
    assert_eq!(total, 520);
}
