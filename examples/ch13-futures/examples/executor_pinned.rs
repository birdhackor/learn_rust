use ch13_futures::runner::PinnedAnswer;
use ch13_futures::runner::run_one;

fn main() {
    let value = run_one(PinnedAnswer::new(0));
    assert_eq!(value, 0);
    println!("沒有Unpin的future，main收到答案={value}");
}
