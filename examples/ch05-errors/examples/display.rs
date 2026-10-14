use ch05_errors::policy::CountError;

fn main() {
    let error = CountError::Negative(-3);
    println!("開發者檢查={error:?}");
    println!("使用者訊息={error}");
    let saved = format!("處理失敗：{error}");
    println!("存成String={}", saved.as_str());
    assert_eq!(saved.as_str(), "處理失敗：件數 -3 不能為負數");
}
