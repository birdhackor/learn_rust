fn main() {
    let free_limit = 300;
    let mut visits = 0;
    let mut shipping_fee = |amount: i32| -> i32 {
        visits += 1;
        if amount >= free_limit { 0 } else { 60 }
    };

    assert_eq!(shipping_fee(280), 60);
    assert_eq!(shipping_fee(320), 0);
    println!("處理訂單數={visits}");
    assert_eq!(visits, 2);
}
