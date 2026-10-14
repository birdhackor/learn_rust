use ch06_iterators::opaque::fixed_reader;
use ch06_iterators::opaque::precise_reader;

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let borrowed = precise_reader(text.as_str());
    println!("明確保留來源 lifetime，bytes={}", borrowed());

    let independent = {
        let mut inside = String::new();
        inside.push_str("hi");
        fixed_reader(inside.as_str())
    };
    println!("明確排除來源 lifetime，內層結束後={}", independent());
    assert_eq!(independent(), 4);
}
