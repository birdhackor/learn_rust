use std::any::Any;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let boxed: Box<dyn Any> = Box::new(text.as_str());
    println!("是否為str reference={}", boxed.is::<&str>());
}
