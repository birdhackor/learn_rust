use std::sync::RwLock;

fn main() {
    let values = RwLock::new(vec![80, 120]);
    let mut view = values.read().expect("新建RwLock尚未poison");
    view.push(30);
}
