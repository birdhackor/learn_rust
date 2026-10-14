use std::any::Any;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let boxed: Box<dyn Any> = Box::new(text);
    match boxed.downcast::<i32>() {
        Ok(number) => println!("取回owned數字={}", *number),
        Err(original) => {
            println!("型別不符，原payload仍可再試");
            match original.downcast::<String>() {
                Ok(mut text) => {
                    text.push_str(" 教學");
                    let taken: String = *text;
                    println!("再取回owned文字={}", taken.as_str());
                }
                Err(_) => println!("原payload也不是String"),
            }
        }
    }
}
