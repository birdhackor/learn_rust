pub mod contracts;

use crate::contracts::require_send;
use std::cell::Cell;

struct Report {
    label: String,
    visits: Cell<i32>,
}

fn main() {
    let mut label = String::new();
    label.push_str("Rust");
    let report = require_send(Report {
        label,
        visits: Cell::new(0),
    });
    report.visits.set(report.visits.get() + 1);
    report.visits.set(report.visits.get() + 1);
    println!(
        "Report通過Send：{}，次數={}",
        report.label.as_str(),
        report.visits.get()
    );
    assert_eq!(report.label.as_str(), "Rust");
    assert_eq!(report.visits.get(), 2);
}
