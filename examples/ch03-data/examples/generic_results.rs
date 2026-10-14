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

fn main() {
    let missing: Option<i32> = None;
    let offered: Option<i32> = Some(120);
    let enabled: Option<bool> = Some(true);

    match missing {
        Some(amount) => println!("報價={amount}"),
        None => println!("缺少數字報價"),
    }
    if let Some(amount) = offered {
        println!("取得報價={amount}");
    }
    if let Some(ready) = enabled {
        println!("設定值={ready}");
    }
    for denominator in [2, 0] {
        match divide(12, denominator) {
            Ok(value) => println!("整數商={value}"),
            Err(DivideError::ZeroDenominator) => println!("除數必須非零"),
        }
    }
}
