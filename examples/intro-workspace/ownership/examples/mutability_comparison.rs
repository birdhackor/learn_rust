fn append(output: &mut String) {
    output.push_str(" 教學");
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust");
    let reference = &mut text;
    append(reference);
    println!("同一份文字={}", reference.as_str());

    let view = text.as_str();
    println!("借讀視圖={view}");
    text.push_str("，完成");
    println!("借讀完成後={}", text.as_str());
}
