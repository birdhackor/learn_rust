use std::sync::Arc;

fn main() {
    let mut values = Arc::new(vec![80]);
    values.push(120);
    println!("長度={}", values.len());
}
