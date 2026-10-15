enum Quote {
    Amount(i32),
    Missing,
}

fn contribution(quote: Quote, minimum: i32) -> i32 {
    match quote {
        Quote::Amount(amount) if amount >= minimum => amount,
        Quote::Amount(_) => 0,
        Quote::Missing => 0,
    }
}

fn selected_count(count: i32, ready: bool) -> i32 {
    match count {
        1 | 2 if ready => count,
        _ => 0,
    }
}

fn main() {
    println!("120通過100門檻={}", contribution(Quote::Amount(120), 100));
    println!("80未過100門檻={}", contribution(Quote::Amount(80), 100));
    println!("缺值貢獻={}", contribution(Quote::Missing, 100));
    println!("1件未備妥={}", selected_count(1, false));
    println!("2件已備妥={}", selected_count(2, true));
    assert_eq!(selected_count(1, false), 0);
    assert_eq!(selected_count(2, true), 2);
}
