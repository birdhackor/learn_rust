struct LabelledOrder {
    label: String,
    count: i32,
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let mut order = LabelledOrder { label, count: 3 };

    let LabelledOrder {
        ref label,
        ref mut count,
    } = order;
    *count += 1;
    println!("借讀標籤={}，借改數量={}", label.as_str(), *count);

    println!("原訂單仍有數量={}", order.count);
}
