mod lifetime_bounds;
mod views;

use crate::lifetime_bounds::Borrowed;
use crate::lifetime_bounds::choose_label;
use crate::lifetime_bounds::require_static;
use crate::views::Wrapper;

fn main() {
    let selected = choose_label("Python", false);
    let borrowed = Borrowed { value: selected };
    println!("literal也能滿足借用契約={}", borrowed.value);

    let mut text = String::new();
    text.push_str("Rust");
    let mut wrapped = require_static(Wrapper { value: text });
    wrapped.push_str(" 教學");
    assert_eq!(wrapped.as_str(), "Rust 教學");
    let taken = wrapped.into_inner();
    println!("owned模型通過static bound={}", taken.as_str());
}
