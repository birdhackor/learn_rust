pub mod runner;

use crate::runner::Recheck;
use crate::runner::run_one;

fn main() {
    let value = run_one(Recheck::new(120));
    assert_eq!(value, 120);
    println!("poll內先通知，main仍收到答案={value}");
}
