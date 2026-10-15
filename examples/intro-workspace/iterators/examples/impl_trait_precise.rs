// --8<-- [start:precise]
fn precise_reader<'a>(text: &'a str) -> impl Fn() -> usize + use<'a> {
    move || text.len()
}

fn fixed_reader(_: &str) -> impl Fn() -> usize + use<> {
    || 4
}
// --8<-- [end:precise]

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
