fn with_local<F>(callback: F) -> i32
where
    F: for<'a> FnOnce(&'a i32) -> i32,
{
    let value = 80;
    callback(&value)
}

fn static_only(value: &'static i32) -> i32 {
    *value
}

fn main() {
    let callback: fn(&'static i32) -> i32 = static_only;
    println!("結果={}", with_local(callback));
}
