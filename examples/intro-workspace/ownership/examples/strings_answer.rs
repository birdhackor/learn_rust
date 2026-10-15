fn append_label(output: &mut String, label: &str) {
    output.push_str(label);
}

fn main() {
    let mut output = String::new();
    append_label(&mut output, "Rust");
    append_label(&mut output, " 教學");
    println!("{}", output.as_str());
}
