fn main() {
    let mut message = String::new();
    message.push_str("Rust");

    let mut messages: Vec<String> = Vec::new();
    messages.push(message);
    let Some(taken) = messages.pop() else {
        return;
    };
    println!("從清單取回={}", taken.as_str());
    println!("清單剩下的項目={}", messages.len());
}
