fn with_local<'a, F>(callback: F) -> i32
where
    F: FnOnce(&'a i32) -> i32,
{
    let value = 80;
    callback(&value)
}

fn main() {
    println!("結果={}", with_local(|number: &i32| *number + 120));
}
