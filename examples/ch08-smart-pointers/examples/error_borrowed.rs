use ch08_smart_pointers::errors::report;
use ch08_smart_pointers::errors::with_context;
use std::error::Error;

fn main() {
    let mut label = String::new();
    label.push_str("資料欄位A");
    match with_context(label.as_str(), "abc") {
        Ok(count) => println!("成功件數={count}"),
        Err(error) => {
            println!("{}", report(&error).as_str());
            match error.source() {
                Some(cause) => println!("原因借自本次error={cause}"),
                None => println!("沒有原因"),
            }
        }
    }
    println!("原標籤仍在={}", label.as_str());
}
