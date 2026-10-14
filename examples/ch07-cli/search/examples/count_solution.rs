use ch07_search::search::search;

fn main() {
    let report = search("Rust", "Rust\nPython\nRust Rust\n", false).expect("搜尋文字非空");
    let total = report
        .matches
        .as_slice()
        .iter()
        .fold(0, |count: usize, _record| count + 1);
    assert_eq!(total, 2);
    println!("符合的行數={total}；同一行的兩次 Rust 仍是一筆結果");
}
