unsafe extern "C" {
    fn tutorial_version() -> u32;
}

fn main() {
    println!("版本={}", tutorial_version());
}
