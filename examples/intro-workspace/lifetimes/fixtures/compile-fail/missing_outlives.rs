fn fallback<'a, 'b>(preferred: Option<&'a str>, backup: &'b str) -> &'a str {
    match preferred {
        Some(text) => text,
        None => backup,
    }
}

fn main() {
    println!("{}", fallback(None, "backup"));
}
