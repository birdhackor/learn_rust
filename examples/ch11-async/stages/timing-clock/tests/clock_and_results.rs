use std::time::Duration;
use tokio::time::sleep;
use tokio::time::timeout;

async fn total_after(subtotal: i32, delay: Duration) -> Result<i32, String> {
    if subtotal < 0 {
        return Err("小計不能為負數".to_owned());
    }
    sleep(delay).await;
    Ok(subtotal + 20)
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn paused_tokio_clock_reaches_the_timer_without_a_wall_clock_assertion() {
    let start = tokio::time::Instant::now();
    sleep(Duration::from_millis(100)).await;
    assert!(start.elapsed().as_millis() >= 100);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn completed_valid_work_keeps_its_inner_ok() {
    let work = total_after(80, Duration::from_millis(10));
    let output = timeout(Duration::from_millis(100), work)
        .await
        .expect("這份受控等待在期限前完成");
    assert_eq!(output, Ok(100));
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn invalid_input_completes_with_an_inner_error_before_waiting() {
    let start = tokio::time::Instant::now();
    let work = total_after(-10, Duration::from_millis(100));
    let output = timeout(Duration::from_millis(10), work)
        .await
        .expect("資料錯誤是已完成的Output");
    match output {
        Err(message) => assert_eq!(message.as_str(), "小計不能為負數"),
        Ok(_) => panic!("負小計應由原工作拒絕"),
    }
    assert_eq!(start.elapsed().as_millis(), 0);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn unfinished_work_returns_the_outer_deadline_error() {
    let work = total_after(80, Duration::from_millis(100));
    match timeout(Duration::from_millis(10), work).await {
        Err(_) => {}
        Ok(_) => panic!("這份受控工作尚未完成，期限已先到"),
    }
}
