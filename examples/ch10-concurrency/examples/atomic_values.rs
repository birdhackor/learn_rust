use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

fn main() {
    let value = AtomicUsize::new(80);
    let old = value.fetch_add(120, Ordering::Relaxed);
    let current = value.load(Ordering::Relaxed);
    assert_eq!(old, 80);
    assert_eq!(current, 200);
    value.store(230, Ordering::Relaxed);
    let replaced = value.load(Ordering::Relaxed);
    assert_eq!(replaced, 230);
    println!("fetch_add交回舊值{old}，更新後{current}；store後{replaced}");
}
