use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct GroupKey {
    region: i32,
    category: i32,
}

fn main() {
    let mut quantities: HashMap<GroupKey, usize> = HashMap::new();
    quantities.insert(
        GroupKey {
            region: 1,
            category: 2,
        },
        3,
    );
    let same = GroupKey {
        region: 1,
        category: 2,
    };
    let different = GroupKey {
        region: 1,
        category: 3,
    };
    let different_region = GroupKey {
        region: 2,
        category: 2,
    };
    match quantities.get(&same) {
        Some(count) => println!("地區1、類別2={}", *count),
        None => println!("地區1、類別2未收錄"),
    }
    match quantities.get(&different) {
        Some(count) => println!("地區1、類別3={}", *count),
        None => println!("地區1、類別3未收錄"),
    }
    match quantities.get(&different_region) {
        Some(count) => println!("地區2、類別2={}", *count),
        None => println!("地區2、類別2未收錄"),
    }
    assert_eq!(quantities.get(&same), Some(&3));
    assert_eq!(quantities.get(&different), None);
    assert_eq!(quantities.get(&different_region), None);
}
