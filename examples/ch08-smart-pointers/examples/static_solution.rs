use ch08_smart_pointers::lifetime_bounds::require_static;
use ch08_smart_pointers::views::Wrapper;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let mut wrapped = require_static(Wrapper { value: text });
    wrapped.push_str(" 教學");
    println!("owned模型通過static bound={}", wrapped.as_str());
}
