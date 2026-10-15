fn main() {
    let missing: Option<i32> = None;
    missing.expect("本案例故意沒有預期中的數字");
}
