fn main() {
    let reference = {
        let inside: i32 = 8;
        &inside
    };
    println!("借用值={}", *reference);
}
