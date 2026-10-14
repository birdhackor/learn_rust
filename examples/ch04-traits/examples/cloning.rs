#[derive(Debug, PartialEq, Clone)]
struct LabelledSummary {
    label: String,
    amount: i32,
}

fn main() {
    let mut label = String::new();
    label.push_str("Rust");
    let original = LabelledSummary { label, amount: 360 };
    let mut duplicate = original.clone();
    assert_eq!(original, duplicate);
    duplicate.label.push_str(" 副本");
    println!("原標籤={}", original.label.as_str());
    println!("副本標籤={}", duplicate.label.as_str());
    assert!(original != duplicate);
    assert_eq!(original.amount, duplicate.amount);

    let views: Vec<&str> = vec![original.label.as_str()];
    let copied_views = views.clone();
    println!(
        "兩份清單借到={}、{}",
        views.as_slice()[0],
        copied_views.as_slice()[0]
    );
    assert_eq!(views.len(), 1);
    assert_eq!(copied_views.len(), 1);
    assert_eq!(copied_views.as_slice()[0], "Rust");
}
