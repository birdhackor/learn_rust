pub mod json_core;
pub mod json_files;

use crate::json_core::codec::decode;
use crate::json_core::codec::encode;
use crate::json_files::NotebookError;
use crate::json_files::read_notebook;

fn archive(filename: &str) -> Result<String, NotebookError> {
    let mut notebook = read_notebook(filename)?;
    notebook.archived = true;
    let json = encode(&notebook)?;
    Ok(json)
}

fn main() {
    let output = archive("fixtures/notebook.json").expect("教材資料可讀取與編碼");
    let changed = decode(output.as_str()).expect("編出的 JSON 符合模型");
    assert_eq!(changed.title.as_str(), "筆記");
    assert_eq!(changed.values.len(), 2);
    assert_eq!(changed.values.as_slice()[0], 80);
    assert_eq!(changed.values.as_slice()[1], 120);
    assert!(changed.archived);
    println!("只有 archived 改為 true：\n{}", output.as_str());
}
