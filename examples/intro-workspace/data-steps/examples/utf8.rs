// --8<-- [start:show-range]
fn show_range(text: &str, start: usize, end: usize) {
    match text.get(start..end) {
        Some(part) => println!("{start}..{end}：有文字「{part}」"),
        None => println!("{start}..{end}：不是有效文字區段"),
    }
}

// --8<-- [end:show-range]
fn main() {
    let marker: char = '中';
    println!("一個 char={marker}");

    let mut text = String::new();
    text.push_str("A中🦀");
    println!("UTF-8 長度={} bytes", text.as_str().len());
    show_range(text.as_str(), 0, 1);
    show_range(text.as_str(), 1, 4);
    show_range(text.as_str(), 4, 8);
    show_range(text.as_str(), 1, 2);
    show_range(text.as_str(), 8, 8);
    show_range(text.as_str(), 4, 9);
}
