fn fallback<'a, 'b>(preferred: Option<&'a str>, backup: &'b str) -> &'a str {
    match preferred {
        Some(text) => text,
        None => backup,
    }
}

fn main() {
    println!("預設備援={}", fallback(None, "backup"));
}
