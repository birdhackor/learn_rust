#[derive(Debug, PartialEq)]
struct LabelledSummary {
    label: String,
    amount: i32,
}

fn main() {
    let mut first_label = String::new();
    first_label.push_str("訂單A");
    let first = LabelledSummary {
        label: first_label,
        amount: 280,
    };
    let mut second_label = String::new();
    second_label.push_str("訂單A");
    let mut second = LabelledSummary {
        label: second_label,
        amount: 280,
    };
    assert_eq!(first, second);
    second.amount += 20;
    assert!(first != second);
    assert_eq!(first.label.as_str(), second.label.as_str());
    assert_eq!(first.amount, 280);
    assert_eq!(second.amount, 300);
    println!("原摘要={first:?}");
    println!("改金額後={second:?}");
}
