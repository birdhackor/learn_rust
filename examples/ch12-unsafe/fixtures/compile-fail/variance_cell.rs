use std::cell::Cell;

fn shorten_cell<'long: 'short, 'short>(value: Cell<&'long str>) -> Cell<&'short str> {
    value
}

fn main() {}
