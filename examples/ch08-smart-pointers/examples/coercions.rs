use ch08_smart_pointers::views::Wrapper;

fn text_bytes(text: &str) -> usize {
    text.len()
}

fn item_count(values: &[i32]) -> usize {
    values.len()
}

fn add_one(value: &mut i32) {
    *value += 1;
}

fn main() {
    let mut text = String::new();
    text.push_str("Rust!");
    let wrapped = Wrapper { value: text };
    println!("顯式借出文字={}", text_bytes(wrapped.value.as_str()));
    println!("自動借用轉換={}", text_bytes(&wrapped));
    println!("文字仍由wrapper持有={}", wrapped.value.as_str());

    let numbers = Wrapper {
        value: vec![80, 120],
    };
    println!("借來的清單長度={}", item_count(&numbers));

    let mut number = Wrapper { value: 80 };
    *number += 20;
    add_one(&mut number);
    println!("透過mutable借用修改={}", *number);
}
