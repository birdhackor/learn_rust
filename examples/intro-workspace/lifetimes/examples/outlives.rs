// --8<-- [start:fallback]
fn fallback<'a, 'b: 'a>(preferred: Option<&'a str>, backup: &'b str) -> &'a str {
    match preferred {
        Some(text) => text,
        None => backup,
    }
}
// --8<-- [end:fallback]

fn main() {
    let mut backup = String::new();
    backup.push_str("backup");
    {
        let mut preferred = String::new();
        preferred.push_str("Rust");
        let present = fallback(Some(preferred.as_str()), backup.as_str());
        println!("有值選到={present}");
        assert!(present.len() == 4);

        let absent: Option<&str> = None;
        let missing = fallback(absent, backup.as_str());
        println!("缺值選到={missing}");
        assert!(missing.len() == 6);

        let empty = fallback(Some(""), backup.as_str());
        let empty_length = empty.len();
        println!("有值但為空，長度={empty_length}");
        assert!(empty_length == 0);
    }
}
