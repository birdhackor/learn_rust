fn stop(message: &str) -> ! {
    panic!("{message}");
}

fn positive(value: i32) -> i32 {
    if value > 0 {
        value
    } else {
        stop("內部契約要求正數")
    }
}

fn main() {
    println!("正常分支的值={}", positive(3));
}
