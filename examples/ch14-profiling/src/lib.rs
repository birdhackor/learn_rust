use ch14_performance::count_borrowed;
use ch14_performance::count_cloned;
use std::hint::black_box;

// --8<-- [start:repeat]
pub fn repeat_counts(lines: &[String], cloned: bool, repetitions: usize) -> usize {
    let mut total_hits = 0;
    for _ in 0..repetitions {
        let query = black_box("Rust");
        let input = black_box(lines);
        let result = if cloned {
            count_cloned(query, input)
        } else {
            count_borrowed(query, input)
        };
        total_hits += black_box(result.expect("本節固定query為非空Rust"));
    }
    total_hits
}
// --8<-- [end:repeat]
