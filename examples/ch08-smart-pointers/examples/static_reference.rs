use ch08_smart_pointers::lifetime_bounds::choose_label;

fn main() {
    let saved: &'static str = {
        let inside: &'static str = "Rust";
        inside
    };
    println!("reference變數結束，literal仍有效={saved}");

    let mut candidate = String::new();
    candidate.push_str("Python");
    let selected = choose_label(candidate.as_str(), false);
    println!("一般借用契約也能回傳literal={selected}");
}
