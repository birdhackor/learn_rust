fn main() {
    let text = "é";
    let scalars: Vec<char> = text.chars().collect();
    println!(
        "bytes={}，scalars={}，內容={scalars:?}",
        text.len(),
        scalars.len()
    );
    assert_eq!(text.len(), 3);
    assert_eq!(scalars.len(), 2);

    let lines: Vec<&str> = "Rust\r\n\nPython\n".lines().collect();
    println!("lines={lines:?}");
    assert_eq!(lines, vec!["Rust", "", "Python"]);
    let empty: Vec<&str> = "".lines().collect();
    assert_eq!(empty.len(), 0);
}
