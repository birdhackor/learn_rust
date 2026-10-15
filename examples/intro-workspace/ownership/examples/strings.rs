// --8<-- [start:show]
fn show(text: &str) {
    println!("借來的文字={text}");
}
// --8<-- [end:show]

fn main() {
    // --8<-- [start:build]
    let mut greeting = String::new();
    greeting.push_str("Hello, ");
    greeting.push_str("Rust");
    show(greeting.as_str());
    // --8<-- [end:build]

    // --8<-- [start:append]
    let ending: &str = "!";
    greeting.push_str(ending);
    show(greeting.as_str());
    // --8<-- [end:append]

    // --8<-- [start:move]
    let moved = greeting;
    show(moved.as_str());
    // --8<-- [end:move]
}
