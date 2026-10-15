fn subtotal(price: i32, count: i32) -> i32 {
    let result = price * count;
    // B：改用 80、2，這次的 result 是 160。
    println!("B：回傳前的小計={result}");
    result
}

fn main() {
    let price: i32 = 80;
    let count: i32 = 2;
    // A：呼叫之前，first 還沒有取得初始值。
    let first = subtotal(price, count);
    // C：呼叫之後，first 是 160。
    println!("第一筆={first}");
}
