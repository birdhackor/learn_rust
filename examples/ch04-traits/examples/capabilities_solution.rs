#[derive(Debug, PartialEq, Clone)]
struct LabelledSummary {
    label: String,
    amount: i32,
}

fn main() {
    let mut label = String::new();
    label.push_str("訂單A");
    let original = LabelledSummary { label, amount: 280 };
    let mut updated = original.clone();
    updated.amount += 20;
    assert_eq!(original.amount, 280);
    assert_eq!(updated.amount, 300);
    assert_eq!(original.label.as_str(), updated.label.as_str());
    assert!(original != updated);
    println!("原資料={original:?}");
    println!("更新資料={updated:?}");
}
