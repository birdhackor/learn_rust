use std::sync::Mutex;

fn require_send<T: Send>(value: T) -> T {
    value
}

fn main() {
    let total = Mutex::new(0);
    let guard = total.lock().expect("新建Mutex尚未poison");
    let _guard = require_send(guard);
}
