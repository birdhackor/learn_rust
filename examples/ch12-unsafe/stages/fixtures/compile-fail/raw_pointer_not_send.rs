fn require_send<T: Send>(value: T) -> T {
    value
}

fn main() {
    let value = 80;
    let pointer = &raw const value;
    let pointer = require_send(pointer);
    println!("pointer是null={}", pointer.is_null());
}
