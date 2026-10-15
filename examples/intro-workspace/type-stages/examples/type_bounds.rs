// --8<-- [start:borrowed]
pub struct Borrowed<'a, T: ?Sized + 'a> {
    pub value: &'a T,
}
// --8<-- [end:borrowed]

fn main() {
    let mut text = String::new();
    text.push_str("Rust");

    let owned_target = Borrowed { value: &text };
    println!("T是String={}", owned_target.value.as_str());

    let unsized_target = Borrowed {
        value: text.as_str(),
    };
    println!("T是str={}", unsized_target.value);

    let reference = text.as_str();
    let nested = Borrowed { value: &reference };
    println!("T是借來的&str={}", *nested.value);

    let boxed_view = Box::new(text.as_str());
    println!("owned Box仍借原文字={}", *boxed_view);
}
