fn main() {
    // --8<-- [start:price]
    let price: i32 = 120;
    let count: i32 = 3;
    let total = price * count;
    println!("總額={total}");
    // --8<-- [end:price]

    // --8<-- [start:division]
    let quotient = 7 / 2;
    let remainder = 7 % 2;
    let decimal: f64 = 7.0 / 2.0;
    println!("整數商={quotient}，餘數={remainder}");
    println!("浮點數商={decimal}");
    // --8<-- [end:division]

    // --8<-- [start:scalars]
    let small: u8 = 255;
    let distance: u32 = 1_000;
    let ready: bool = true;
    let grade: char = '甲';
    println!("小整數={small}，距離={distance}");
    println!("已備妥={ready}，等級={grade}");
    // --8<-- [end:scalars]
}
