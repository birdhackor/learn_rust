use ch08_smart_pointers::views::Wrapper;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let mut wrapped = Wrapper { value: text };
    wrapped.push_str(" 教學");
    assert_eq!(wrapped.as_str(), "Rust 教學");
    let taken = wrapped.into_inner();
    println!("取回owned文字={}", taken.as_str());
}
