use std::collections::HashMap;

// --8<-- [start:lines]
fn nonempty_names(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter(|line: &&str| !(*line).is_empty())
}
// --8<-- [end:lines]

// --8<-- [start:counts]
fn count_names(text: &str) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for name in nonempty_names(text) {
        counts
            .entry(name.to_owned())
            .and_modify(|count: &mut usize| *count += 1)
            .or_insert(1);
    }
    counts
}
// --8<-- [end:counts]

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
    let mut owned_total = 0;
    for (name, count) in counts.into_iter() {
        println!("接手名稱={}，次數={count}", name.as_str());
        owned_total += count;
    }
    println!("接手後合計={owned_total}");
    assert_eq!(owned_total, 23);
}
