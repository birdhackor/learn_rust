fn main() {
    let mut values = vec![1, 2];
    for item in &values {
        println!("for &Vec 取得 shared item={}", *item);
    }
    for item in &mut values {
        *item += 10;
    }
    println!("原第一項修改後={}", values.as_slice()[0]);
    assert_eq!(values.as_slice()[0], 11);
    assert_eq!(values.as_slice()[1], 12);
}
