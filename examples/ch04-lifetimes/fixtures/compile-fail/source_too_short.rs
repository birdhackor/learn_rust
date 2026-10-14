fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() { first } else { second }
}

fn main() {
    let mut outside = String::new();
    outside.push_str("Rust tutorial");
    let selected = {
        let mut inside = String::new();
        inside.push_str("hi");
        longer(outside.as_str(), inside.as_str())
    };
    println!("內層結束後={selected}");
}
