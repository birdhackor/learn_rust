fn main() {
    let first: i32 = 3;
    let second: i32 = 8;
    let mut reference: &i32 = &first;
    println!("第一次讀={}", *reference);
    reference = &second;
    println!("換一個借用對象後={}", *reference);
    println!("原資料仍是 {first} 和 {second}");
}
