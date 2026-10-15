pub mod runner;

use crate::runner::PinnedAnswer;
use crate::runner::run_one;

fn main() {
    let value = run_one(PinnedAnswer::new(0));
    assert_eq!(value, 0);
    println!("沒有Unpin的future，main收到答案={value}");
}
