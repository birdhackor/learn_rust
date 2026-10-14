struct Amount {
    value: i32,
}

fn main() {
    let mut first: Box<i32> = Box::new(120);
    let view = &*first;
    println!("借用Box中的數字={}", *view);
    let writable = &mut *first;
    *writable += 20;

    let second = first;
    println!("接手Box後的數字={}", *second);

    let boxed: Box<Amount> = Box::new(Amount { value: 80 });
    let taken: Amount = *boxed;
    println!("從Box取回的資料={}", taken.value);
}
