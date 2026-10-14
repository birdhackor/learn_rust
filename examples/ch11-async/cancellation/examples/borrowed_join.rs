use ch11_cancellation::borrowed_join_demo;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let outcome = borrowed_join_demo().await;
    println!("停止分支先返回；原 task 仍正常完成：{outcome:?}");
}
