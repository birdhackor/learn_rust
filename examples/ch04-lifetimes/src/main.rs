use ch04_lifetimes::source::longer;

fn main() {
    let mut first = String::new();
    first.push_str("Rust");
    let mut second = String::new();
    second.push_str("Python");
    let selected = longer(first.as_str(), second.as_str());
    println!("byte 較長的是={selected}");

    let same_result = longer(second.as_str(), first.as_str());
    println!("交換參數後={same_result}");
}
