use std::collections::HashMap;

#[derive(PartialEq, Eq)]
struct GroupKey {
    region: i32,
    category: i32,
}

fn main() {
    let mut counts: HashMap<GroupKey, usize> = HashMap::new();
    counts.insert(
        GroupKey {
            region: 1,
            category: 2,
        },
        3,
    );
}
