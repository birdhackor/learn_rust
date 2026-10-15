pub mod contracts;

use crate::contracts::require_send;
use crate::contracts::require_sync;

struct Batch {
    label: String,
    values: Vec<i32>,
}

fn main() {
    let mut label = String::new();
    label.push_str("Rust");
    let batch = Batch {
        label,
        values: vec![80, 120],
    };
    let batch = require_send(batch);
    let view = require_sync(&batch);
    let checked_reference = require_send(view);

    let mut total = 0;
    for value in checked_reference.values.as_slice() {
        total += *value;
    }
    println!(
        "通過兩項契約：{}有{}項，小計={total}",
        checked_reference.label.as_str(),
        checked_reference.values.len()
    );
    assert_eq!(total, 200);
    assert_eq!(batch.values.len(), 2);
}
