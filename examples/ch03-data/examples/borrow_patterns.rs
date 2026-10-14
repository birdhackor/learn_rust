enum Quote {
    Missing,
    Amount(i32),
    Rejected { code: i32 },
}

fn is_available(quote: &Quote) -> bool {
    match quote {
        Quote::Missing => false,
        Quote::Amount(amount) => *amount >= 0,
        Quote::Rejected { code } => {
            println!("拒絕代碼={}", *code);
            false
        }
    }
}

fn main() {
    let quote = Quote::Amount(120);
    println!("可用={}", is_available(&quote));
    println!("再次讀取，可用={}", is_available(&quote));
    assert!(is_available(&quote));
    assert!(!is_available(&Quote::Missing));
    assert!(!is_available(&Quote::Rejected { code: 37 }));
}
