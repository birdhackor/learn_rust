use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn store_with(ordering: Ordering) {
    let count = AtomicUsize::new(0);
    count.store(1, ordering);
}

fn main() {
    store_with(Ordering::AcqRel);
}
