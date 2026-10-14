use ch11_timing::two_steps;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn each_whole_budget_leaves_the_expected_number_of_completed_steps() {
    for (step_ms, limit_ms, expected) in [(20, 50, 2), (30, 50, 1), (30, 10, 0)] {
        let mut completed = 0;
        let work = two_steps(&mut completed, Duration::from_millis(step_ms));
        let result = timeout(Duration::from_millis(limit_ms), work).await;
        assert_eq!(completed, expected);
        match result {
            Ok(()) => assert_eq!(completed, 2),
            Err(_) => assert!(completed < 2),
        }
    }
}
