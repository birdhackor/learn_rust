fn can_ship(price: i32, count: i32, ready: bool) -> bool {
    price >= 0 && count > 0 && ready
}

fn needs_preparation(ready: bool) -> bool {
    !ready
}

fn main() {
    let first = can_ship(80, 2, true);
    let second = can_ship(80, 0, true);
    let third = can_ship(-1, 2, true);
    let waiting = needs_preparation(false);
    println!("可出貨={first}，空單可出貨={second}，負單價可出貨={third}");
    println!("尚需備貨={waiting}");
}
