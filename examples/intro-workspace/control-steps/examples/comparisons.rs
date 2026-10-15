// --8<-- [start:count-check]
fn positive_count(count: i32) -> bool {
    println!("檢查數量={count}");
    count > 0
}
// --8<-- [end:count-check]

// --8<-- [start:short-circuit]
fn both_checks(price: i32, count: i32) -> bool {
    price >= 0 && positive_count(count)
}

fn either_check(ready: bool, count: i32) -> bool {
    ready || positive_count(count)
}
// --8<-- [end:short-circuit]

fn main() {
    // --8<-- [start:comparisons]
    let subtotal: i32 = 300;
    let equal = subtotal == 300;
    let different = subtotal != 300;
    let below = subtotal < 300;
    let at_most = subtotal <= 300;
    let above = subtotal > 300;
    let at_least = subtotal >= 300;
    println!("等於={equal}，不等於={different}");
    println!("小於={below}，小於等於={at_most}");
    println!("大於={above}，大於等於={at_least}");
    // --8<-- [end:comparisons]

    // --8<-- [start:logic]
    let price: i32 = 120;
    let count: i32 = 0;
    let valid = price >= 0 && count >= 0;
    let empty = count == 0;
    let has_items = !empty;
    println!("材料非負={valid}，有商品={has_items}");
    // --8<-- [end:logic]

    println!("A：兩項都要成立，單價為負");
    let first = both_checks(-1, 2);
    println!("A的結果={first}");
    println!("B：兩項都要成立，單價非負");
    let second = both_checks(120, 2);
    println!("B的結果={second}");
    println!("C：任一項成立，已備妥");
    let third = either_check(true, 0);
    println!("C的結果={third}");
    println!("D：任一項成立，尚未備妥");
    let fourth = either_check(false, 0);
    println!("D的結果={fourth}");
}
