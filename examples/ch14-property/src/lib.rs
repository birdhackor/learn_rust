use std::collections::HashMap;

// --8<-- [start:frequencies]
pub fn frequencies(values: &[i32]) -> HashMap<i32, usize> {
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for value in values {
        match counts.get_mut(value) {
            Some(count) => *count += 1,
            None => {
                counts.insert(*value, 1);
            }
        }
    }
    counts
}

pub fn count_of(counts: &HashMap<i32, usize>, key: i32) -> usize {
    match counts.get(&key) {
        Some(count) => *count,
        None => 0,
    }
}
// --8<-- [end:frequencies]

// --8<-- [start:total-count]
/// 借讀本節frequencies產生的次數，總和等於原輸入筆數。
pub fn total_count(counts: &HashMap<i32, usize>) -> usize {
    let mut total = 0;
    for count in counts.values() {
        total += *count;
    }
    total
}
// --8<-- [end:total-count]
