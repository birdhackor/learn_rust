struct LabelledOrder {
    label: String,
    unit_price: i32,
    count: i32,
}

enum Quote {
    Missing,
    Offered(LabelledOrder),
    Rejected { code: i32 },
}

fn show_status(quote: &Quote) {
    match quote {
        Quote::Missing => println!("尚無報價"),
        Quote::Offered(order) => println!("收到報價：{}", order.label.as_str()),
        Quote::Rejected { code } => println!("報價遭拒絕，代碼={}", *code),
    }
}

fn total_if_offered(quote: &Quote) -> i32 {
    let Quote::Offered(order) = quote else {
        return 0;
    };
    if order.count == 0 {
        return 0;
    }
    let goods = order.unit_price * order.count;
    let shipping = match goods {
        0..=299 => 20,
        _ => 0,
    };
    goods + shipping
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let quotes = [
        Quote::Offered(LabelledOrder {
            label,
            unit_price: 80,
            count: 2,
        }),
        Quote::Missing,
        Quote::Rejected { code: 37 },
    ];
    for quote in &quotes[..] {
        show_status(quote);
        println!("本次計價={}", total_if_offered(quote));
    }
    assert_eq!(total_if_offered(&quotes[0]), 180);
}
