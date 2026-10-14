fn main() {
    let mut count: i32 = 3;
    let reference = &count;
    println!("先讀借用值={}", *reference);
    count += 1;
    println!("再修改原值={count}");

    let saved = {
        let inside: i32 = 8;
        inside
    };
    assert_eq!(saved, 8);
    println!("保存到外層的是 owned 數值={saved}");
}
