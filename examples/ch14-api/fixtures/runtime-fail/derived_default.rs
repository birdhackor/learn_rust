#[derive(Default)]
struct BatchLimit(usize);

fn main() {
    let limit = BatchLimit::default();
    assert!(limit.0 >= 1, "預設必須符合非零規則");
}
