fn main() {
    let free_limit = 300;
    let shipping_fee = |amount: i32| -> i32 { if amount >= free_limit { 0 } else { 60 } };

    println!("280 元訂單運費={}", shipping_fee(280));
    println!("320 元訂單運費={}", shipping_fee(320));
    assert_eq!(shipping_fee(300), 0);
}
