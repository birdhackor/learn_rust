fn main() {
    let identity = |value| value;
    let number: i32 = identity(8);
    let flag: bool = identity(true);
    println!("{number}、{flag}");
}
