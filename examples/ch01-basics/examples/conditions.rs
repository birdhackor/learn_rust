fn total_with_shipping(price: i32, count: i32) -> i32 {
    if count == 0 {
        return 0;
    }
    let subtotal = price * count;
    let shipping = if subtotal >= 300 { 0 } else { 20 };
    subtotal + shipping
}

fn main() {
    let small = total_with_shipping(80, 2);
    let large = total_with_shipping(120, 3);
    let empty = total_with_shipping(120, 0);
    println!("小訂單={small}，大訂單={large}，空訂單={empty}");
    let price = 120;
    let count = 3;
    let valid = price >= 0 && count >= 0;
    if valid {
        println!("材料符合非負數條件");
    } else {
        println!("材料不符合非負數條件");
    }
}
