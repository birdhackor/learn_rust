fn apply_shared<F: Fn(i32) -> i32>(task: &F, value: i32) -> i32 {
    task(value)
}

fn add_one(value: i32) -> i32 {
    value + 1
}

fn main() {
    let operation: fn(i32) -> i32 = add_one;
    let double: fn(i32) -> i32 = |value| value * 2;
    println!("function pointer 呼叫={}", operation(3));
    println!("不捕捉環境的 closure 轉成 pointer={}", double(3));
    println!("共用 Fn 介面也接受 pointer={}", apply_shared(&operation, 3));
}
