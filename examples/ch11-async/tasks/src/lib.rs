// --8<-- [start:model]
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::task::Id;
use tokio::task::JoinSet;

// --8<-- [start:work-error]
#[derive(Debug, PartialEq)]
pub enum WorkError {
    Negative(i32),
    ControlClosed,
}
// --8<-- [end:work-error]

// --8<-- [start:outcome-model]
#[derive(Debug, PartialEq)]
pub enum Outcome {
    Value(i32),
    WorkError(WorkError),
    Panic,
    Cancelled,
    OtherTaskFailure(String),
}

#[derive(Debug, PartialEq)]
pub struct Report {
    pub input: usize,
    pub outcome: Outcome,
}
// --8<-- [end:outcome-model]
// --8<-- [end:model]

// --8<-- [start:controlled-work]
pub async fn controlled_work(
    amount: i32,
    panic_after_release: bool,
    started: mpsc::Sender<()>,
    mut release: mpsc::Receiver<()>,
) -> Result<i32, WorkError> {
    started
        .send(())
        .await
        .expect("本例 controller 持有啟動訊息接收端");
    drop(started);
    match release.recv().await {
        Some(()) => (),
        None => return Err(WorkError::ControlClosed),
    }
    if panic_after_release {
        panic!("task 故意 panic");
    }
    if amount < 0 {
        Err(WorkError::Negative(amount))
    } else {
        Ok(amount)
    }
}
// --8<-- [end:controlled-work]

// --8<-- [start:collect-one]
pub async fn collect_one(
    tasks: &mut JoinSet<Result<i32, WorkError>>,
    inputs: &mut HashMap<Id, usize>,
) -> Option<Report> {
    let result = match tasks.join_next_with_id().await {
        Some(result) => result,
        None => return None,
    };
    let (id, outcome) = match result {
        Ok((id, Ok(value))) => (id, Outcome::Value(value)),
        Ok((id, Err(error))) => (id, Outcome::WorkError(error)),
        Err(error) => {
            let id = error.id();
            let outcome = if error.is_cancelled() {
                Outcome::Cancelled
            } else if error.is_panic() {
                Outcome::Panic
            } else {
                Outcome::OtherTaskFailure(error.to_string())
            };
            (id, outcome)
        }
    };
    let input = inputs
        .remove(&id)
        .expect("每份 task 在 spawn 後已記錄輸入位置");
    Some(Report { input, outcome })
}
// --8<-- [end:collect-one]

// --8<-- [start:demo-setup]
pub async fn run_demo() -> Vec<Report> {
    let (started_tx, mut started_rx) = mpsc::channel::<()>(4);
    let (release_0, receiver_0) = mpsc::channel::<()>(1);
    let (release_1, receiver_1) = mpsc::channel::<()>(1);
    let (release_2, receiver_2) = mpsc::channel::<()>(1);
    let (release_3, receiver_3) = mpsc::channel::<()>(1);
    let mut tasks = JoinSet::new();
    let mut inputs = HashMap::new();

    let first = tasks.spawn(controlled_work(80, false, started_tx.clone(), receiver_0));
    inputs.insert(first.id(), 0);
    let second = tasks.spawn(controlled_work(-1, false, started_tx.clone(), receiver_1));
    inputs.insert(second.id(), 1);
    let third = tasks.spawn(controlled_work(120, true, started_tx.clone(), receiver_2));
    inputs.insert(third.id(), 2);
    let fourth = tasks.spawn(controlled_work(200, false, started_tx.clone(), receiver_3));
    inputs.insert(fourth.id(), 3);
    drop(started_tx);

    let mut started_count = 0;
    while let Some(()) = started_rx.recv().await {
        started_count += 1;
    }
    assert_eq!(started_count, 4);
    // --8<-- [end:demo-setup]

    // --8<-- [start:demo-collect]
    let mut reports = Vec::new();
    release_1.send(()).await.expect("input 1 仍持有接收端");
    reports.push(
        collect_one(&mut tasks, &mut inputs)
            .await
            .expect("仍有 input 1"),
    );
    release_2.send(()).await.expect("input 2 仍持有接收端");
    reports.push(
        collect_one(&mut tasks, &mut inputs)
            .await
            .expect("仍有 input 2"),
    );
    fourth.abort();
    reports.push(
        collect_one(&mut tasks, &mut inputs)
            .await
            .expect("仍有 input 3"),
    );
    match release_3.send(()).await {
        Err(_) => (),
        Ok(()) => panic!("已等待取消，input 3 的接收端應已清理"),
    }
    release_0.send(()).await.expect("input 0 仍持有接收端");
    reports.push(
        collect_one(&mut tasks, &mut inputs)
            .await
            .expect("仍有 input 0"),
    );
    match collect_one(&mut tasks, &mut inputs).await {
        None => (),
        Some(_) => panic!("四份 task 都已回收，不應還有其他結果"),
    }
    assert_eq!(inputs.len(), 0);
    reports
}
// --8<-- [end:demo-collect]

// --8<-- [start:tests]
#[cfg(test)]
mod tests {
    use crate::Outcome;
    use crate::WorkError;
    use crate::controlled_work;
    use crate::run_demo;
    use tokio::sync::mpsc;

    #[tokio::test(flavor = "current_thread")]
    async fn task_starts_before_its_join_handle_is_awaited() {
        let (started_tx, mut started_rx) = mpsc::channel::<()>(1);
        let (release_tx, release_rx) = mpsc::channel::<()>(1);
        let handle = tokio::spawn(controlled_work(80, false, started_tx, release_rx));
        assert_eq!(started_rx.recv().await, Some(()));
        assert!(!handle.is_finished());
        release_tx.send(()).await.expect("task 仍持有接收端");
        match handle.await {
            Ok(Ok(value)) => assert_eq!(value, 80),
            other => panic!("本例應正常回傳金額：{other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn completion_order_and_all_failures_keep_the_original_input() {
        let reports = run_demo().await;
        assert_eq!(reports.len(), 4);
        let reports = reports.as_slice();
        assert_eq!(reports[0].input, 1);
        assert_eq!(
            reports[0].outcome,
            Outcome::WorkError(WorkError::Negative(-1))
        );
        assert_eq!(reports[1].input, 2);
        assert_eq!(reports[1].outcome, Outcome::Panic);
        assert_eq!(reports[2].input, 3);
        assert_eq!(reports[2].outcome, Outcome::Cancelled);
        assert_eq!(reports[3].input, 0);
        assert_eq!(reports[3].outcome, Outcome::Value(80));
    }
}
// --8<-- [end:tests]
