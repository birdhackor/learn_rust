use ch06_iterators::text_ops::report;

fn main() {
    let mut text = String::new();
    text.push_str("Rust\nPython\nRust\n\n教學\n");
    let rows = report(text.as_str());
    for (name, count) in rows {
        println!("{}={count}", name.as_str());
    }
    assert_eq!(text.as_str().lines().collect::<Vec<&str>>().len(), 5);
}
