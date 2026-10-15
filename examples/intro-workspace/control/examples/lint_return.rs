// 本節刻意保留尾端 return，與下面的尾端 expression 比較。
#[allow(clippy::needless_return)]
fn explicit_subtotal(price: i32, count: i32) -> i32 {
    return price * count;
}

fn tail_subtotal(price: i32, count: i32) -> i32 {
    price * count
}

fn main() {
    let explicit = explicit_subtotal(80, 2);
    let tail = tail_subtotal(80, 2);
    println!("return版本={explicit}，尾端expression版本={tail}");
}
