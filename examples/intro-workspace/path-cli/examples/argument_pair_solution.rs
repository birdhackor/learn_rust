use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum PairError {
    MissingFilename,
    TooManyArguments,
}

fn parse_pair<I: IntoIterator<Item = OsString>>(input: I) -> Result<(PathBuf, PathBuf), PairError> {
    let mut arguments = input.into_iter();
    let _program_name = arguments.next();
    let first = match arguments.next() {
        Some(value) => value,
        None => return Err(PairError::MissingFilename),
    };
    let second = match arguments.next() {
        Some(value) => value,
        None => return Err(PairError::MissingFilename),
    };
    match arguments.next() {
        Some(_) => Err(PairError::TooManyArguments),
        None => Ok((PathBuf::from(first), PathBuf::from(second))),
    }
}

fn main() {
    let (first, second) = parse_pair([
        OsString::from("pair"),
        OsString::from("中文筆記.txt"),
        OsString::from("second.txt"),
    ])
    .expect("固定資料含兩個檔名");
    assert_eq!(first, PathBuf::from("中文筆記.txt"));
    assert_eq!(second, PathBuf::from("second.txt"));
    match parse_pair([OsString::from("pair"), OsString::from("one.txt")]) {
        Err(PairError::MissingFilename) => {}
        _ => panic!("應回報缺少第二個檔名"),
    }
    match parse_pair([
        OsString::from("pair"),
        OsString::from("one.txt"),
        OsString::from("two.txt"),
        OsString::from("extra.txt"),
    ]) {
        Err(PairError::TooManyArguments) => {}
        _ => panic!("應回報超量參數"),
    }
    println!("兩個檔名都保留原生路徑，缺少與過量參數分開回報");
}
