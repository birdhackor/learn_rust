use std::cell::Ref;
use std::cell::RefCell;

// --8<-- [start:models]
pub struct Numbers {
    pub values: Vec<i32>,
}

pub struct GuardedNumbers {
    pub values: RefCell<Vec<i32>>,
}
// --8<-- [end:models]

// --8<-- [start:trait]
pub trait BorrowView {
    type View<'a>
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a>;
}
// --8<-- [end:trait]

// --8<-- [start:impls]
impl BorrowView for Numbers {
    type View<'a>
        = &'a [i32]
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a> {
        self.values.as_slice()
    }
}

impl BorrowView for GuardedNumbers {
    type View<'a>
        = Ref<'a, Vec<i32>>
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a> {
        self.values.borrow()
    }
}
// --8<-- [end:impls]

// --8<-- [start:helper]
pub fn borrow_view<S: BorrowView>(source: &S) -> S::View<'_> {
    source.view()
}
// --8<-- [end:helper]
