#[cfg(feature = "batch")]
fn two_orders(first: [i32; 2], second: [i32; 2]) -> i32 {
    first[0] * first[1] + second[0] * second[1]
}

fn main() {
    if cfg!(feature = "batch") {
        let total = two_orders([80, 2], [120, 1]);
        println!("兩筆商品小計={total}");
    }
}
