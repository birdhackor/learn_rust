fn call_twice<F: FnMut() -> i32>(mut task: F) -> (i32, i32) {
    (task(), task())
}

fn main() {
    let mut count = 0;
    let tick = || {
        count += 1;
        count
    };
    let (first, second) = call_twice(tick);
    println!("兩次結果={first}、{second}，原計數={count}");
    assert_eq!(count, 2);
    let (first_constant, second_constant) = call_twice(|| 7);
    assert_eq!(first_constant, 7);
    assert_eq!(second_constant, 7);
}
