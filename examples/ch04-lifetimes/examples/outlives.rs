use ch04_lifetimes::source::fallback;

fn main() {
    let mut backup = String::new();
    backup.push_str("backup");
    {
        let mut preferred = String::new();
        preferred.push_str("Rust");
        let chosen = fallback(Some(preferred.as_str()), backup.as_str());
        println!("有值選到={chosen}");

        let absent: Option<&str> = None;
        let chosen = fallback(absent, backup.as_str());
        println!("缺值選到={chosen}");
    }
}
