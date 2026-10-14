fn borrowed_bytes<T>(value: &T) -> usize {
    std::mem::size_of_val(value)
}

fn main() {
    let word: &str = "Rust!";
    println!("{}", borrowed_bytes(word));
}
