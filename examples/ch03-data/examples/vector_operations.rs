fn main() {
    let mut values: Vec<i32> = Vec::new();
    values.push(80);
    values.push(120);
    values.push(200);
    println!("新增後的長度={}", values.len());

    match values.as_slice().get(1) {
        Some(value) => println!("索引1的數字={}", *value),
        None => println!("沒有索引1"),
    }
    match values.as_slice().get(9) {
        Some(value) => println!("索引9的數字={}", *value),
        None => println!("沒有索引9"),
    }

    let removed = values.remove(0);
    println!("移除索引0的數字={removed}");
    println!("剩下的第一項={}", values.as_slice()[0]);

    match values.pop() {
        Some(value) => println!("取回最後一項={value}"),
        None => println!("清單已空"),
    }

    let writable: &mut [i32] = values.as_mut_slice();
    writable[0] += 10;
    println!("修改剩下的第一項={}", values.as_slice()[0]);

    let initial: Vec<i32> = vec![120, 80, 200];
    println!("vec! 建立的長度={}", initial.len());
}
