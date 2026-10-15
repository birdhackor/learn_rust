use std::any::Any;

fn inspect(value: &dyn Any) {
    println!("確切型別是i32={}", value.is::<i32>());
    match value.downcast_ref::<i32>() {
        Some(number) => println!("借回i32={}", *number),
        None => println!("這份資料不是i32"),
    }
}

fn main() {
    let amount: i32 = 80;
    let count: usize = 80;
    inspect(&amount);
    inspect(&count);
    println!("原資料仍是amount={amount}、count={count}");

    let mut text = String::new();
    text.push_str("Rust");
    let view: &dyn Any = &text;
    match view.downcast_ref::<String>() {
        Some(original) => println!("局部String也能借成Any={}", original.as_str()),
        None => println!("不是String"),
    }
}
