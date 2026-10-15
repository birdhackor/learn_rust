struct Holder<T> {
    value: T,
}

// --8<-- [start:implementation]
impl<T> Holder<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn into_value(self) -> T {
        self.value
    }
}
// --8<-- [end:implementation]

fn main() {
    // --8<-- [start:number]
    let number: Holder<i32> = Holder::new(120);
    let saved = number.into_value();
    assert_eq!(saved, 120);
    println!("接回數字={saved}");
    // --8<-- [end:number]

    let mut label = String::new();
    label.push_str("訂單A");
    let text: Holder<String> = Holder::new(label);
    let saved_label = text.into_value();
    println!("接回文字={}", saved_label.as_str());
}
