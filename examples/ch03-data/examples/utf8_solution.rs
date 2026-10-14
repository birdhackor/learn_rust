use ch03_data::text::selected_byte_len;

fn show_length(text: &str, start: usize, end: usize) {
    match selected_byte_len(text, start, end) {
        Some(length) => println!("{start}..{end}：取得 {length} bytes"),
        None => println!("{start}..{end}：無效區段"),
    }
}

fn main() {
    show_length("A中🦀", 1, 4);
    show_length("A中🦀", 1, 2);
    show_length("A中🦀", 8, 8);
}
