fn total_with_shipping(price: i32, count: i32) -> i32 {
    // --8<-- [start:early-return]
    if count == 0 {
        return 0;
    }
    // --8<-- [end:early-return]
    // --8<-- [start:shipping-value]
    let item_total = price * count;
    let shipping = if item_total >= 300 { 0 } else { 20 };
    // --8<-- [end:shipping-value]
    item_total + shipping
}

fn main() {
    let small = total_with_shipping(80, 2);
    let boundary = total_with_shipping(100, 3);
    let large = total_with_shipping(120, 3);
    let empty = total_with_shipping(120, 0);
    println!("小訂單={small}，門檻訂單={boundary}");
    println!("大訂單={large}，空訂單={empty}");

    show_selected_price(3);
}

// --8<-- [start:index-check]
fn show_selected_price(index: usize) {
    let prices: [i32; 3] = [120, 80, 200];
    if index < prices.len() {
        let selected = prices[index];
        println!("選到價格={selected}");
    } else {
        println!("索引無效，這次不讀取元素");
    }
}
// --8<-- [end:index-check]
