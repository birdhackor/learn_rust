use std::cell::Cell;

fn copied_cell<'long: 'short, 'short>(source: &Cell<&'long str>) -> Cell<&'short str> {
    Cell::new(source.get())
}

fn main() {
    let original: Cell<&'static str> = Cell::new("Rust");
    let copied = copied_cell(&original);
    let mut local = String::new();
    local.push_str("note");
    copied.set(local.as_str());
    println!("新Cell可改成本地借用={}", copied.get());
    println!("原Cell仍儲存static借用={}", original.get());
    assert_eq!(original.get(), "Rust");
}
