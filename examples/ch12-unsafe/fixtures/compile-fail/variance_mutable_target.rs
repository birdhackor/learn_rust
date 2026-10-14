fn replace<T>(target: &mut T, value: T) {
    *target = value;
}

fn main() {
    let mut saved: &'static str = "Rust";
    {
        let mut local = String::new();
        local.push_str("note");
        replace(&mut saved, local.as_str());
    }
    println!("之後仍想讀saved={saved}");
}
