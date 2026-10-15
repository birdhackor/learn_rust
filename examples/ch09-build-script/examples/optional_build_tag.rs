fn main() {
    let tag: Option<&'static str> = option_env!("LESSON_OPTIONAL_TAG");
    match tag {
        Some(text) => println!("編譯時的額外標記={text}"),
        None => println!("編譯時沒有額外標記"),
    }
}
