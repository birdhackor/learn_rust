fn main() {
    let values = vec![10, 20];
    let mut visits = 0;
    let mapped = values.as_slice().iter().map(|item: &i32| {
        visits += 1;
        println!("map 處理={}", *item);
        *item * 2
    });
    println!("建立完成，尚未走訪");
    for value in mapped {
        println!("for 收到={value}");
    }
    println!("實際 callback 次數={visits}");
    assert_eq!(visits, 2);
    assert_eq!(values.len(), 2);
}
