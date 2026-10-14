fn take_positive(value: i32) -> i32 {
    if value <= 0 {
        panic!("內部契約要求正數，實際值={value}");
    }
    value
}

fn main() {
    println!("取出的值={}", take_positive(0));
}
