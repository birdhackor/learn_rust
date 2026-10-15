fn apply_shared<F: Fn(i32) -> i32>(task: &F, value: i32) -> i32 {
    task(value)
}

fn main() {
    let base = 10;
    let add = |value| value + base;
    println!("shared 借用呼叫={}", apply_shared(&add, 3));
    println!("保留原工作再呼叫={}", add(4));
    assert_eq!(apply_shared(&add, 0), 10);
}
