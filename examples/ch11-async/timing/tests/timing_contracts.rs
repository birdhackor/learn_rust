use ch11_timing::total_after;
use ch11_timing::two_steps;
use std::time::Duration;
use tokio::time::sleep;
use tokio::time::timeout;

// --8<-- [start:clock]
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn paused_tokio_clock_reaches_the_timer_without_a_wall_clock_assertion() {
    let start = tokio::time::Instant::now();
    sleep(Duration::from_millis(100)).await;
    assert!(start.elapsed().as_millis() >= 100);
}
// --8<-- [end:clock]

// --8<-- [start:result-paths]
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
// --8<-- [end:result-paths]

// --8<-- [start:scope]
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn one_limit_covers_both_steps_and_keeps_the_completed_first_step() {
    let mut completed = 0;
    let work = two_steps(&mut completed, Duration::from_millis(30));
    match timeout(Duration::from_millis(50), work).await {
        Err(_) => {}
        Ok(()) => panic!("整段預算不足以完成兩個等待"),
    }
    assert_eq!(completed, 1);
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn each_step_gets_a_new_limit_when_wrapped_separately() {
    let mut completed = 0;
    for _ in 0..2 {
        let step = async {
            sleep(Duration::from_millis(30)).await;
            completed += 1;
        };
        timeout(Duration::from_millis(50), step)
            .await
            .expect("這一次只限制單一步驟");
    }
    assert_eq!(completed, 2);
}
// --8<-- [end:scope]
