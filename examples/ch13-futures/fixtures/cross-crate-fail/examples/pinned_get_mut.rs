use ch13_futures::pinning::PinnedNote;
use std::mem;

fn main() {
    let mut left = PinnedNote::new(80);
    let mut right = PinnedNote::new(120);
    mem::swap(left.as_mut().get_mut(), right.as_mut().get_mut());
}
