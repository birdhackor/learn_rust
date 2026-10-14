use clap::Parser;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "query-settings", version = "0.1.0")]
struct Settings {
    query: String,
    filename: PathBuf,
    #[arg(long)]
    ignore_case: bool,
    #[arg(long)]
    line_numbers: bool,
}

fn main() {
    let ordinary = Settings::try_parse_from([
        OsString::from("query-settings"),
        OsString::from("Rust"),
        OsString::from("中文筆記.txt"),
    ])
    .expect("固定參數未要求行號");
    assert!(!ordinary.line_numbers);

    let numbered = Settings::try_parse_from([
        OsString::from("query-settings"),
        OsString::from("Rust"),
        OsString::from("中文筆記.txt"),
        OsString::from("--line-numbers"),
    ])
    .expect("固定參數要求已宣告的行號開關");
    assert!(numbered.line_numbers);
    assert!(!numbered.ignore_case);
    assert_eq!(numbered.query.as_str(), "Rust");
    assert_eq!(numbered.filename, PathBuf::from("中文筆記.txt"));
    println!("line_numbers 由 false 變成 true，其他設定保留");
}
