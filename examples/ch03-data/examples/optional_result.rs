enum DivideError {
    ZeroDenominator,
}

fn main() {
    let offered = Some(120);
    match offered {
        Some(amount) => println!("有報價={amount}"),
        None => println!("沒有報價"),
    }

    let available = false;
    let missing = if available { Some(120) } else { None };
    match missing {
        Some(amount) => println!("有報價={amount}"),
        None => println!("沒有報價"),
    }

    for denominator in [2, 0] {
        let divided = if denominator == 0 {
            Err(DivideError::ZeroDenominator)
        } else {
            Ok(12 / denominator)
        };
        match divided {
            Ok(value) => println!("整數商={value}"),
            Err(DivideError::ZeroDenominator) => println!("不能用零作除數"),
        }
    }
}
