fn make_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

fn main() {
    let mut counter = make_counter(1);
    let first = counter();
    let second = counter();
    println!("工廠交回的計數工作={first}、{second}");
    assert_eq!(first, 2);
    assert_eq!(second, 3);
    let mut another = make_counter(10);
    assert_eq!(another(), 11);
}
