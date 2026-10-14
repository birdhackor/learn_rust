use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let second = Duration::from_secs(1);
    let delay = Duration::from_millis(20);
    println!("1秒的總毫秒數={}", second.as_millis());
    println!("設定等待={} ms", delay.as_millis());

    let start = Instant::now();
    sleep(delay).await;
    let elapsed = start.elapsed();
    println!("這次量測的經過時間={} ms", elapsed.as_millis());
}
