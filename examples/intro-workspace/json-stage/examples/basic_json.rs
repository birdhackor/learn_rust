pub mod json_core;

use crate::json_core::codec::decode;
use crate::json_core::codec::encode;
use std::fs;

fn main() {
    let text = match fs::read_to_string("fixtures/notebook.json") {
        Ok(text) => text,
        Err(error) => {
            println!("讀取失敗：{error}");
            return;
        }
    };
    let notebook = match decode(text.as_str()) {
        Ok(notebook) => notebook,
        Err(error) => {
            println!("JSON 解析失敗：{error}");
            return;
        }
    };
    println!(
        "標題={}，資料項數={}",
        notebook.title.as_str(),
        notebook.values.len()
    );
    match encode(&notebook) {
        Ok(json) => {
            println!("重新編成 JSON：");
            println!("{}", json.as_str());
        }
        Err(error) => println!("JSON 編碼失敗：{error}"),
    }
}
