fn main() {
    let mut count = 0;
    let mut tick = || {
        count += 1;
        count
    };
    println!("尚未完成借用時={count}");
    println!("之後仍呼叫={}", tick());
}
