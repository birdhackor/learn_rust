use ch08_smart_pointers::lifetime_bounds::Borrowed;

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
}
