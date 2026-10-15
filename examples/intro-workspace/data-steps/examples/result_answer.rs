enum AmountError {
    Negative,
}

fn checked_amount(amount: i32) -> Result<i32, AmountError> {
    if amount < 0 {
        Err(AmountError::Negative)
    } else {
        Ok(amount)
    }
}

fn main() {
    for amount in [0, -5] {
        match checked_amount(amount) {
            Ok(value) => {
                assert_eq!(value, 0);
                println!("接受金額={value}");
            }
            Err(AmountError::Negative) => {
                assert!(amount < 0);
                println!("拒絕負數金額");
            }
        }
    }
}
