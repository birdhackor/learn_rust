use std::sync::RwLock;

fn require_send<T: Send>(value: T) -> T {
    value
}

fn main() {
    let first = RwLock::new(80);
    let second = RwLock::new(120);
    let reader = first.read().expect("新建RwLock尚未poison");
    let writer = second.write().expect("另一份新建RwLock尚未poison");
    let _reader = require_send(reader);
    let _writer = require_send(writer);
}
