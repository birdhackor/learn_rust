fn main() {
    let mut total: i32 = 160;
    total += 20;
    println!("加入運費={total}");
    total -= 10;
    println!("折扣後={total}");

    let total = total * 2;
    println!("新階段總額={total}");
}
