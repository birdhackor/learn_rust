fn main() {
    // --8<-- [start:plain-break]
    let mut remaining = 2;
    loop {
        if remaining == 0 {
            break;
        }
        remaining -= 1;
    }
    println!("停止後剩餘={remaining}");
    // --8<-- [end:plain-break]

    // --8<-- [start:break-value]
    let mut current = 0;
    let found = loop {
        current += 1;
        if current >= 3 {
            break current;
        }
    };
    println!("loop交出的值={found}");
    // --8<-- [end:break-value]

    // --8<-- [start:continue]
    let mut total = 0;
    for number in 0..=3 {
        if number == 2 {
            continue;
        }
        total += number;
    }
    println!("跳過2的總和={total}");
    // --8<-- [end:continue]
}
