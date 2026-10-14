use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut new_groups = 0;
    for name in ["Rust", "Python", "Rust"] {
        counts
            .entry(name.to_owned())
            .and_modify(|count: &mut usize| *count += 1)
            .or_insert_with(|| {
                new_groups += 1;
                1
            });
    }
    println!("建立新群組的 callback 次數={new_groups}");
    assert_eq!(new_groups, 2);
    assert_eq!(counts.get("Rust"), Some(&2));
    assert_eq!(counts.get("Python"), Some(&1));
}
