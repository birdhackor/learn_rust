struct Amount {
    value: i32,
}

fn main() {
    // --8<-- [start:number-box]
    let mut first: Box<i32> = Box::new(120);
    let view = &*first;
    println!("借讀 Box 中的數字={}", *view);
    let writable = &mut *first;
    *writable += 20;

    let second = first;
    println!("接手 Box 後的數字={}", *second);
    assert_eq!(*second, 140);
    // --8<-- [end:number-box]

    // --8<-- [start:take-owned]
    let boxed: Box<Amount> = Box::new(Amount { value: 80 });
    let taken: Amount = *boxed;
    println!("從 Box 取回的資料={}", taken.value);
    assert_eq!(taken.value, 80);
    // --8<-- [end:take-owned]
}
