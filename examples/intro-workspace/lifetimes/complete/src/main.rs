use intro_lifetimes_complete::source::longer;
use intro_lifetimes_complete::views::make_view;

fn main() {
    let mut first = String::new();
    first.push_str("Rust");
    let mut second = String::new();
    second.push_str("Python");

    let selected = longer(first.as_str(), second.as_str());
    let view = make_view(selected);
    println!("借到較長文字={}，bytes={}", view.text(), view.byte_len());
    assert_eq!(view.original(), "Python");
}
