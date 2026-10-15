use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn read_with(ordering: Ordering) -> usize {
    let count = AtomicUsize::new(0);
    count.load(ordering)
}

fn main() {
    let _count = read_with(Ordering::Release);
}
