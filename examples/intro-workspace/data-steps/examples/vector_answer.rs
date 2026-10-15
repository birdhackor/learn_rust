fn main() {
    let mut values: Vec<i32> = vec![10, 20];
    values.push(30);
    let removed = values.remove(0);
    assert_eq!(removed, 10);
    println!("移除第一項={removed}");

    let writable: &mut [i32] = values.as_mut_slice();
    writable[0] += 5;
    println!("留下的第一項={}", values.as_slice()[0]);

    for _ in 0..3 {
        match values.pop() {
            Some(value) => println!("取回末尾={value}"),
            None => println!("清單已空"),
        }
    }
    let remaining = values.len();
    assert!(remaining == 0);
    println!("剩餘項目={remaining}");
}
