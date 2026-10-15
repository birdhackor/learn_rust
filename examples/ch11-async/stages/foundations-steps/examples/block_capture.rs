#[tokio::main(flavor = "current_thread")]
async fn main() {
    let heading = "Rust".to_owned();
    let borrowed_work = async {
        println!("async block 本體：借用 heading");
        heading.as_str().len()
    };
    println!("block 已建立，原文字仍可讀：{}", heading.as_str());
    let bytes = borrowed_work.await;
    assert_eq!(bytes, 4);
    println!("借用工作的 Output：{bytes}");

    let note = "配送".to_owned();
    let owned_work = async move { note };
    println!("owned 工作已接手文字");
    let saved = owned_work.await;
    assert_eq!(saved.as_str(), "配送");
    println!("owned 工作交回文字：{}", saved.as_str());
}
