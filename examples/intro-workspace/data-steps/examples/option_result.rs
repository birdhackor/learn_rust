// --8<-- [start:divide]
enum DivideError {
    ZeroDenominator,
}

fn divide(numerator: i32, denominator: i32) -> Result<i32, DivideError> {
    if denominator == 0 {
        Err(DivideError::ZeroDenominator)
    } else {
        Ok(numerator / denominator)
    }
}
// --8<-- [end:divide]

// --8<-- [start:show-option]
fn show_amount(offered: Option<i32>) {
    match offered {
        Some(amount) => println!("取得報價={amount}"),
        None => println!("缺少報價"),
    }
}
// --8<-- [end:show-option]

fn main() {
    // --8<-- [start:options]
    let missing: Option<i32> = None;
    let offered: Option<i32> = Some(120);
    let enabled: Option<bool> = Some(true);
    // --8<-- [end:options]
    show_amount(missing);
    show_amount(offered);
    if let Some(ready) = enabled {
        println!("設定值={ready}");
    }

    // --8<-- [start:choose-option]
    for available in [true, false] {
        let selected = if available { Some(0) } else { None };
        show_amount(selected);
    }
    // --8<-- [end:choose-option]

    // --8<-- [start:handle-result]
    for denominator in [2, 0] {
        match divide(12, denominator) {
            Ok(value) => println!("整數商={value}"),
            Err(DivideError::ZeroDenominator) => println!("除數必須非零"),
        }
    }
    // --8<-- [end:handle-result]
}
