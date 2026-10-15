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

#[cfg(test)]
mod tests {
    use crate::gat::GuardedNumbers;
    use crate::gat::Numbers;
    use crate::gat::borrow_view;
    use std::cell::RefCell;
    use std::mem::drop;

    #[test]
    fn slice_view_borrows_the_original_numbers() {
        let mut source = Numbers {
            values: vec![80, 120],
        };
        let view: &[i32] = borrow_view(&source);
        assert_eq!(view[0] + view[1], 200);
        source.values.push(40);
        let next: &[i32] = borrow_view(&source);
        assert_eq!(next.len(), 3);
        assert_eq!(next[2], 40);
    }

    #[test]
    fn guard_view_keeps_the_runtime_borrow_until_drop() {
        let source = GuardedNumbers {
            values: RefCell::new(vec![80]),
        };
        let view = borrow_view(&source);
        assert_eq!(view.as_slice()[0], 80);
        assert!(source.values.try_borrow_mut().is_err());
        drop(view);
        {
            let mut values = source.values.borrow_mut();
            values.push(120);
        }
        let next = borrow_view(&source);
        assert_eq!(next.as_slice()[0] + next.as_slice()[1], 200);
    }

    #[test]
    fn both_views_can_borrow_empty_storage() {
        let plain = Numbers { values: Vec::new() };
        let guarded = GuardedNumbers {
            values: RefCell::new(Vec::new()),
        };
        assert_eq!(borrow_view(&plain).len(), 0);
        assert_eq!(borrow_view(&guarded).len(), 0);
    }
}
