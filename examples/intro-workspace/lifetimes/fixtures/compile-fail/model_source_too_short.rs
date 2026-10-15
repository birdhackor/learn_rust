struct TextView<'a> {
    text: &'a str,
}

fn main() {
    let view = {
        let mut inside = String::new();
        inside.push_str("Rust");
        TextView {
            text: inside.as_str(),
        }
    };
    println!("{}", view.text);
}
