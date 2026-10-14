fn main() {
    let mut count = 0;
    let mut tick = || {
        count += 1;
        count
    };

    println!("第一次計數={}", tick());
    println!("第二次計數={}", tick());
    println!("最後一次使用 tick 後，原計數={count}");
    assert_eq!(count, 2);
}
