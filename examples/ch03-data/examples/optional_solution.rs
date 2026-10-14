enum AmountError {
    Negative,
}

fn main() {
    for amount in [0, -5] {
        let checked = if amount < 0 {
            Err(AmountError::Negative)
        } else {
            Ok(amount)
        };
        match checked {
            Ok(value) => println!("接受金額={value}"),
            Err(AmountError::Negative) => println!("拒絕負數金額"),
        }
    }
}
