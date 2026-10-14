use ch05_errors::policy::CountError;
use std::borrow::ToOwned;
use std::string::ToString;

fn main() {
    let error = CountError::Negative(-1);
    let message: String = error.to_string();
    assert_eq!(message.as_str(), "件數 -1 不能為負數");
    println!("保存顯示文字：{}", message.as_str());
    println!("原錯誤仍可顯示：{error}");

    let mut original = String::new();
    original.push_str("Rust");
    let mut copied: String = original.as_str().to_owned();
    copied.push_str(" 教學");
    assert_eq!(original.as_str(), "Rust");
    assert_eq!(copied.as_str(), "Rust 教學");
    println!(
        "原文字：{}；owned 複本：{}",
        original.as_str(),
        copied.as_str()
    );

    let values = [10, 20];
    let borrowed_values: &[i32] = &values[..];
    let mut copied_values: Vec<i32> = borrowed_values.to_owned();
    copied_values.push(30);
    assert_eq!(values.len(), 2);
    assert_eq!(copied_values.len(), 3);
    println!("原 array 有 2 筆；owned Vec 有 3 筆");
}
