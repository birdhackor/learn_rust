use ch08_smart_pointers::newtypes::Wrapped;

fn main() {
    let wrapped: Wrapped<i32> = 80.into();
    println!("本地wrapper提供From，取回原數字={}", wrapped.into_inner());
    let mut text = String::new();
    text.push_str("Rust");
    let wrapped = Wrapped::from(text);
    let recovered = wrapped.into_inner();
    println!("owned文字也能整體交回={}", recovered.as_str());
}
