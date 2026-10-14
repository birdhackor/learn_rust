use ch06_iterators::text_ops::count_names;

fn main() {
    let mut counts = count_names("Rust\nPython\nRust");
    let mut total = 0;
    for (name, count) in counts.iter() {
        println!("借用名稱={}，次數={}", name.as_str(), *count);
        total += *count;
    }
    assert_eq!(total, 3);
    for (name, count) in counts.iter_mut() {
        *count += 10;
        println!("保留原鍵={}，更新次數={}", name.as_str(), *count);
    }
    println!("修改後 Rust 次數={:?}", counts.get("Rust"));
    assert_eq!(counts.get("Rust"), Some(&12));
    assert_eq!(counts.get("Python"), Some(&11));
}
