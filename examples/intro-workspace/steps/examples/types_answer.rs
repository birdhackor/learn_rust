fn main() {
    let price: i32 = 80;
    let count: i32 = 2;
    let shipping: i32 = 20;
    let total = price * count + shipping;
    let quotient = 9 / 2;
    let remainder = 9 % 2;
    println!("含運總額={total}");
    println!("整數商={quotient}，餘數={remainder}");
}
