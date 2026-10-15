fn shipping(tier: i32) -> i32 {
    match tier {
        0 => 20,
        _ => true,
    }
}

fn main() {
    println!("{}", shipping(0));
}
