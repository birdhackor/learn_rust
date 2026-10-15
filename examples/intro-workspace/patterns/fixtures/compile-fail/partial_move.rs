struct LabelledOrder {
    label: String,
    count: i32,
}

fn show(order: &LabelledOrder) {
    println!("{}：{}件", order.label.as_str(), order.count);
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let order = LabelledOrder { label, count: 3 };
    let LabelledOrder { label, .. } = order;
    println!("{}", label.as_str());
    show(&order);
}
