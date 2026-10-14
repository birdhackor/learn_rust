struct Labelled {
    label: String,
}

impl Drop for Labelled {
    fn drop(&mut self) {
        println!("清理標籤={}", self.label.as_str());
    }
}

fn main() {
    let mut label = String::new();
    label.push_str("Rust");
    let value = Labelled { label };
    let taken = value.label;
    println!("取出的文字={}", taken.as_str());
}
