use std::time::Duration;
use std::time::Instant;
use tokio::time::timeout;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let work = async {
        std::thread::sleep(Duration::from_millis(30));
        100
    };

    let start = Instant::now();
    let result = timeout(Duration::from_millis(1), work).await;
    match result {
        Ok(value) => println!("阻塞工作仍完成：{value}"),
        Err(_) => panic!("沒有交還控制權的本體已一次完成"),
    }
    println!("這次阻塞範圍經過={} ms", start.elapsed().as_millis());
}
