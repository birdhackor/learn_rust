enum Quote {
    Missing,
    Amount(i32),
}

fn with_match(quote: &Quote) {
    let amount = match quote {
        Quote::Amount(amount) => amount,
        Quote::Missing => {
            println!("match：略過缺值");
            return;
        }
    };
    println!("match：含運={}", *amount + 20);
}

fn with_if_let(quote: &Quote) {
    if let Quote::Amount(amount) = quote {
        println!("if let：含運={}", *amount + 20);
    } else {
        println!("if let：略過缺值");
    }
}

// --8<-- [start:let-else]
fn with_let_else(quote: &Quote) {
    let Quote::Amount(amount) = quote else {
        println!("let else：略過缺值");
        return;
    };
    println!("let else：含運={}", *amount + 20);
}
// --8<-- [end:let-else]

fn main() {
    let quote = Quote::Amount(120);
    with_match(&quote);
    with_if_let(&quote);
    with_let_else(&quote);
    with_match(&Quote::Missing);
    with_if_let(&Quote::Missing);
    with_let_else(&Quote::Missing);
}
