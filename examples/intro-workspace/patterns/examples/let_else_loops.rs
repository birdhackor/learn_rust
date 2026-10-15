enum Quote {
    Missing,
    Amount(i32),
}

fn skip_missing(quotes: &[Quote]) -> i32 {
    let mut total = 0;
    for quote in quotes {
        let Quote::Amount(amount) = quote else {
            continue;
        };
        total += *amount;
    }
    total
}

fn stop_at_missing(quotes: &[Quote]) -> i32 {
    let mut total = 0;
    for quote in quotes {
        let Quote::Amount(amount) = quote else {
            break;
        };
        total += *amount;
    }
    total
}

fn main() {
    let quotes = [Quote::Amount(120), Quote::Missing, Quote::Amount(80)];
    let skipped = skip_missing(&quotes[..]);
    let stopped = stop_at_missing(&quotes[..]);
    assert_eq!(skipped, 200);
    assert_eq!(stopped, 120);
    println!("略過缺值後合計={skipped}，遇缺值停止合計={stopped}");
}
