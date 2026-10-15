fn main() {
    let mut amount = 80;
    let pointer: *mut i32 = &raw mut amount;

    // SAFETY: pointer直接來自仍存活且已初始化的mutable i32局部值，
    // 位置對i32正確aligned。存取期間沒有其他reference或pointer讀寫它，
    // 本例只有單一thread；也沒有移動amount或釋放其storage。
    let (before, after) = unsafe {
        let before = *pointer;
        *pointer = 100;
        (before, *pointer)
    };

    println!("raw read取得={before}，raw write後={after}");
    println!("raw操作結束後原值={amount}");
    assert_eq!(amount, 100);
}
