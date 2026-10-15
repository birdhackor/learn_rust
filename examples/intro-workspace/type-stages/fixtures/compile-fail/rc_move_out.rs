use std::rc::Rc;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let owner = Rc::new(text);
    let taken = *owner;
    println!("文字={}", taken.as_str());
}
