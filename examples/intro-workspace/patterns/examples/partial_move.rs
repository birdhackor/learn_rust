struct LabelledOrder {
    label: String,
    count: i32,
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let order = LabelledOrder { label, count: 3 };

    let LabelledOrder { label, .. } = order;
    println!("取出的文字={}", label.as_str());
    println!("尚未移出的 count={}", order.count);
    assert_eq!(order.count, 3);
}
