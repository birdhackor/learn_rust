fn with_local<F, T>(callback: F) -> T
where
    F: for<'a> FnOnce(&'a i32) -> T,
{
    let value = 80;
    callback(&value)
}

fn main() {
    let saved = with_local(|number: &i32| number);
    println!("借出的值={saved}");
}
