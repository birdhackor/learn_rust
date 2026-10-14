fn main() {
    let price: i32 = 120;
    let count: i32 = 3;
    let total = {
        let subtotal = price * count;
        subtotal + 20
    };
    println!("含運費的總額={total}");
}
