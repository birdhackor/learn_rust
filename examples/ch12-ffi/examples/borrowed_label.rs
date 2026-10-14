use ch12_ffi::is_rust_label;

fn main() {
    println!(
        "Rust完全相符={}",
        is_rust_label("Rust").expect("沒有內部零")
    );
    println!(
        "Python完全相符={}",
        is_rust_label("Python").expect("沒有內部零")
    );
    match is_rust_label("Ru\0st") {
        Ok(answer) => println!("判定={answer}"),
        Err(_) => println!("內部零被Rust拒絕，尚未呼叫C"),
    }
}
