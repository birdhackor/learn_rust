fn subtotal(price: i32, count: i32) -> i32 {
    let result = price * count;
    // B：回傳前，本次 subtotal 已算出 result。
    println!("B：回傳前的小計={result}");
    result
}

fn main() {
    let price: i32 = 120;
    let count: i32 = 3;
    // A：呼叫之前，first 還沒有取得初始值。
    let first = subtotal(price, count);
    // C：呼叫之後，first 是 360。
    println!("第一筆={first}");
    let second = subtotal(80, 2);
    println!("第二筆={second}");
}
