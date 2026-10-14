use std::ffi::CString;

fn main() {
    let saved = {
        let word = CString::new("Rust").expect("沒有內部零");
        word.as_c_str()
    };
    println!("文字={:?}", saved.to_str());
}
