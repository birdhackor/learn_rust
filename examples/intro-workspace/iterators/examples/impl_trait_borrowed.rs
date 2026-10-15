// --8<-- [start:borrowed]
fn byte_reader(text: &str) -> impl Fn() -> usize {
    move || text.len()
}
// --8<-- [end:borrowed]

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let read = byte_reader(text.as_str());
    println!("回傳工作借原文字，bytes={}", read());
    println!("來源仍存在，原文字={}", text.as_str());
}
