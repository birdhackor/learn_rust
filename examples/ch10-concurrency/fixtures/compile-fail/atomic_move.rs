use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn main() {
    let first = AtomicUsize::new(80);
    let second = first;
    println!("第二份名稱讀到{}", second.load(Ordering::Relaxed));
    println!("原名稱仍想讀{}", first.load(Ordering::Relaxed));
}
