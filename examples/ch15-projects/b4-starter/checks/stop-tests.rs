#[cfg(test)]
mod tests {
    use crate::Batch;
    use crate::Completion;
    use crate::ConfigError;
    use crate::EndReason;
    use crate::Scheduler;
    use crate::StopEvent;
    use crate::input::Input;
    use crate::input::prepare_inputs;
    use crate::run_batch;
    use crate::single::Check;
    use crate::single::Outcome;
    use crate::single::build_client;
    use crate::single::default_limit;
    use ch11_local_service::Fixture;
    use std::time::Duration;
    use tokio::sync::mpsc;
    use tokio::time::error::Elapsed;
    use tokio::time::timeout;

    // --8<-- [start:cleanup]
    async fn drain(scheduler: &mut Scheduler) {
        scheduler.tasks.abort_all();
        while let Some(_result) = scheduler.tasks.join_next_with_id().await {}
    }

    async fn finish(
        mut scheduler: Scheduler,
        fixture: Fixture,
        waited: Result<EndReason, Elapsed>,
    ) -> (Batch, usize) {
        // 測試自己的後備cleanup不能代替被驗函式的完成責任。
        let tracked = scheduler.tasks.len();
        let registered = scheduler.inputs.len();
        drain(&mut scheduler).await;
        let stopped = fixture.shutdown().await;
        let hits = stopped.expect("server已收尾");
        let reason = waited.expect("有限觀察已取得批次理由");
        assert_eq!(tracked, 0);
        assert_eq!(registered, 0);
        (scheduler.into_batch(reason), hits)
    }
    // --8<-- [end:cleanup]

    // --8<-- [start:control]
    async fn make_scheduler(
        client: &reqwest::Client,
        inputs: Vec<Input>,
        fixture: Fixture,
    ) -> (Scheduler, Fixture) {
        match Scheduler::new(client, inputs, 2, default_limit()) {
            Ok(scheduler) => (scheduler, fixture),
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("合法材料應建立排程：{error:?}；server={stopped:?}");
            }
        }
    }

    async fn two_entered(fixture: &mut Fixture) -> Result<Vec<usize>, String> {
        let mut seen = Vec::new();
        for _ in 0..2 {
            match timeout(Duration::from_secs(5), fixture.next_started()).await {
                Ok(Some(index)) => seen.push(index),
                other => return Err(format!("held通知缺席：{other:?}")),
            }
        }
        Ok(seen)
    }

    async fn controlled_stop(
        mut scheduler: Scheduler,
        mut fixture: Fixture,
    ) -> (Batch, usize, Vec<usize>) {
        let (stop_tx, mut stop_rx) = mpsc::channel::<StopEvent>(1);
        let mut observed = None;
        let waited = timeout(Duration::from_secs(15), async {
            tokio::select! {
                _ = async {
                    observed = Some(two_entered(&mut fixture).await);
                    let _sent = stop_tx.send(StopEvent::Requested).await;
                    std::future::pending::<()>().await;
                } => panic!("控制支路的pending不會完成"),
                reason = scheduler.run(&mut stop_rx) => reason,
            }
        })
        .await;
        let (batch, hits) = finish(scheduler, fixture, waited).await;
        let seen = observed.expect("控制支路應有觀察").expect("兩份held已進入");
        (batch, hits, seen)
    }
    // --8<-- [end:control]

    fn assert_cancelled(entry: &crate::Entry) {
        match &entry.completion {
            Completion::Cancelled { original: _ } => (),
            other => panic!("應為Cancelled：{other:?}"),
        }
    }

    fn assert_unstarted(entry: &crate::Entry) {
        match &entry.completion {
            Completion::Unstarted { original: _ } => (),
            other => panic!("應為Unstarted：{other:?}"),
        }
    }

    fn assert_pair(seen: &[usize]) {
        assert_eq!(seen.len(), 2);
        assert!((seen[0] == 0 && seen[1] == 1) || (seen[0] == 1 && seen[1] == 0));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn configuration_errors_are_rejected_before_prequeued_stop() {
        let client = build_client(true).expect("client");
        for jobs in [0, 2] {
            let (stop_tx, stop_rx) = mpsc::channel::<StopEvent>(1);
            stop_tx.send(StopEvent::Requested).await.expect("收端存在");
            let result = run_batch(&client, Vec::new(), jobs, default_limit(), stop_rx).await;
            let expected = if jobs == 0 {
                ConfigError::ZeroJobs
            } else {
                ConfigError::Empty
            };
            match result {
                Err(error) => assert_eq!(error, expected),
                other => panic!("設定錯誤先拒絕，不能回Requested：{other:?}"),
            }
        }
    }

    // --8<-- [start:prequeued-test]
    #[tokio::test(flavor = "current_thread")]
    async fn prequeued_stop_does_not_submit_even_the_first_batch() {
        let client = build_client(true).expect("client");
        let fixture = Fixture::start(0).await.expect("ready");
        let address = fixture.url("/ok");
        let text = format!("\n{address}\n \n/notes\n{address}\n");
        let inputs = match prepare_inputs(text.as_str()) {
            Ok(inputs) => inputs,
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("固定非空材料：{error:?}；server={stopped:?}");
            }
        };
        let (mut scheduler, fixture) = make_scheduler(&client, inputs, fixture).await;
        let (stop_tx, mut stop_rx) = mpsc::channel::<StopEvent>(1);
        stop_tx.send(StopEvent::Requested).await.expect("收端存在");
        let waited = timeout(Duration::from_secs(5), scheduler.run(&mut stop_rx)).await;
        let (batch, hits) = finish(scheduler, fixture, waited).await;
        assert_eq!(batch.reason, EndReason::Requested);
        assert_eq!(hits, 0);
        let indexes: Vec<usize> = batch
            .entries
            .iter()
            .map(|entry| entry.input_index)
            .collect();
        assert_eq!(indexes, vec![1, 3, 4]);
        let rows = batch.entries.as_slice();
        assert_eq!(rows[0].original(), address.as_str());
        assert_eq!(rows[1].original(), "/notes");
        assert_eq!(rows[2].original(), address.as_str());
        for row in rows {
            assert_unstarted(row);
        }
    }
    // --8<-- [end:prequeued-test]

    #[tokio::test(flavor = "current_thread")]
    async fn requested_stop_keeps_raw_gaps_duplicates_and_cancels_only_submitted() {
        let client = build_client(true).expect("client");
        let fixture = Fixture::start(2).await.expect("ready");
        let first = fixture.url("/held/0");
        let second = fixture.url("/held/1");
        let text = format!("\n{first}\n \n{second}\n{first}\n/notes\n");
        let inputs = match prepare_inputs(text.as_str()) {
            Ok(inputs) => inputs,
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("四份非空材料：{error:?}；server={stopped:?}");
            }
        };
        let (scheduler, fixture) = make_scheduler(&client, inputs, fixture).await;
        let (batch, hits, seen) = controlled_stop(scheduler, fixture).await;
        assert_pair(seen.as_slice());
        assert_eq!(batch.reason, EndReason::Requested);
        assert_eq!(hits, 2);
        let indexes: Vec<usize> = batch
            .entries
            .iter()
            .map(|entry| entry.input_index)
            .collect();
        assert_eq!(indexes, vec![1, 3, 4, 5]);
        let rows = batch.entries.as_slice();
        assert_cancelled(&rows[0]);
        assert_cancelled(&rows[1]);
        assert_unstarted(&rows[2]);
        assert_unstarted(&rows[3]);
        assert_eq!(rows[0].original(), first.as_str());
        assert_eq!(rows[1].original(), second.as_str());
        assert_eq!(rows[2].original(), first.as_str());
        assert_eq!(rows[3].original(), "/notes");
    }

    // --8<-- [start:refill-test]
    #[tokio::test(flavor = "current_thread")]
    async fn stop_after_refill_preserves_completed_http_and_pending_inputs() {
        let client = build_client(true).expect("client");
        let fixture = Fixture::start(2).await.expect("ready");
        let first = fixture.url("/held/0");
        let ok = fixture.url("/ok");
        let second = fixture.url("/held/1");
        let text = format!("\n{first}\n \n{ok}\n{second}\n/notes\n{ok}\n");
        let inputs = match prepare_inputs(text.as_str()) {
            Ok(inputs) => inputs,
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("五份非空材料：{error:?}；server={stopped:?}");
            }
        };
        let (scheduler, fixture) = make_scheduler(&client, inputs, fixture).await;
        let (batch, hits, seen) = controlled_stop(scheduler, fixture).await;
        assert_pair(seen.as_slice());
        assert_eq!(batch.reason, EndReason::Requested);
        assert_eq!(hits, 3);
        let indexes: Vec<usize> = batch
            .entries
            .iter()
            .map(|entry| entry.input_index)
            .collect();
        assert_eq!(indexes, vec![1, 3, 4, 5, 6]);
        let rows = batch.entries.as_slice();
        assert_cancelled(&rows[0]);
        match &rows[1].completion {
            Completion::Checked(Check {
                outcome:
                    Outcome::Http {
                        status,
                        passed,
                        final_url,
                    },
                original: _,
                elapsed_ms: _,
            }) => {
                assert_eq!(*status, 200);
                assert!(*passed);
                assert_eq!(final_url.as_str(), ok.as_str());
            }
            other => panic!("已完成HTTP仍保留：{other:?}"),
        }
        assert_cancelled(&rows[2]);
        assert_unstarted(&rows[3]);
        assert_unstarted(&rows[4]);
        assert_eq!(rows[0].original(), first.as_str());
        assert_eq!(rows[1].original(), ok.as_str());
        assert_eq!(rows[2].original(), second.as_str());
        assert_eq!(rows[3].original(), "/notes");
        assert_eq!(rows[4].original(), ok.as_str());
    }
    // --8<-- [end:refill-test]

    // --8<-- [start:completed-test]
    #[tokio::test(flavor = "current_thread")]
    async fn drain_preserves_uncollected_check_and_panic_then_classifies_the_rest() {
        let client = build_client(true).expect("client");
        let pending = vec![Input {
            input_index: 9,
            original: "pending-original".to_owned(),
        }];
        let mut scheduler = Scheduler::new(&client, pending, 3, default_limit()).expect("合法材料");
        let (done_tx, mut done_rx) = mpsc::channel::<()>(2);
        let first_done = done_tx.clone();
        let first = scheduler.tasks.spawn(async move {
            let _sent = first_done.send(()).await;
            Check {
                original: "completed-original".to_owned(),
                elapsed_ms: None,
                outcome: Outcome::Timeout,
            }
        });
        scheduler.inputs.insert(
            first.id(),
            Input {
                input_index: 1,
                original: "completed-original".to_owned(),
            },
        );
        let failed = scheduler.tasks.spawn(async move {
            let _sent = done_tx.send(()).await;
            panic!("受控的已完成task故障");
        });
        scheduler.inputs.insert(
            failed.id(),
            Input {
                input_index: 3,
                original: "panic-original".to_owned(),
            },
        );
        let (held_tx, mut held_rx) = mpsc::channel::<()>(1);
        let held = scheduler.tasks.spawn(async move {
            let _release = held_rx.recv().await;
            Check {
                original: "held-original".to_owned(),
                elapsed_ms: None,
                outcome: Outcome::Timeout,
            }
        });
        scheduler.inputs.insert(
            held.id(),
            Input {
                input_index: 6,
                original: "held-original".to_owned(),
            },
        );
        let observed = timeout(Duration::from_secs(5), async {
            let first = done_rx.recv().await;
            let second = done_rx.recv().await;
            (first, second)
        })
        .await;
        let waited = timeout(Duration::from_secs(5), scheduler.stop_and_finish()).await;
        let tracked = scheduler.tasks.len();
        let registered = scheduler.inputs.len();
        drain(&mut scheduler).await;
        let closed = match held_tx.send(()).await {
            Err(_) => true,
            Ok(()) => false,
        };
        match observed {
            Ok((Some(()), Some(()))) => (),
            other => panic!("合成完成通知應齊全：{other:?}"),
        }
        waited.expect("收尾觀察有期限");
        assert_eq!(tracked, 0);
        assert_eq!(registered, 0);
        assert!(closed);
        let batch = scheduler.into_batch(EndReason::Requested);
        let indexes: Vec<usize> = batch
            .entries
            .iter()
            .map(|entry| entry.input_index)
            .collect();
        assert_eq!(indexes, vec![1, 3, 6, 9]);
        let rows = batch.entries.as_slice();
        match &rows[0].completion {
            Completion::Checked(_) => (),
            other => panic!("完成尚未收取的Check不能覆寫：{other:?}"),
        }
        match &rows[1].completion {
            Completion::TaskFailure { error, original: _ } => assert!(error.is_panic()),
            other => panic!("完成的task故障不能覆寫Cancelled：{other:?}"),
        }
        assert_cancelled(&rows[2]);
        assert_unstarted(&rows[3]);
        assert_eq!(rows[0].original(), "completed-original");
        assert_eq!(rows[1].original(), "panic-original");
        assert_eq!(rows[2].original(), "held-original");
        assert_eq!(rows[3].original(), "pending-original");
    }
    // --8<-- [end:completed-test]

    #[tokio::test(flavor = "current_thread")]
    async fn failed_signal_and_closed_source_have_distinct_reasons() {
        let client = build_client(true).expect("client");
        for failed in [true, false] {
            let inputs = vec![Input {
                input_index: 4,
                original: "/notes".to_owned(),
            }];
            let mut scheduler =
                Scheduler::new(&client, inputs, 2, default_limit()).expect("合法材料");
            let (stop_tx, mut stop_rx) = mpsc::channel::<StopEvent>(1);
            if failed {
                stop_tx
                    .send(StopEvent::SignalFailed("受控監聽錯誤".to_owned()))
                    .await
                    .expect("收端存在");
            }
            drop(stop_tx);
            let waited = timeout(Duration::from_secs(5), scheduler.run(&mut stop_rx)).await;
            let tracked = scheduler.tasks.len();
            let registered = scheduler.inputs.len();
            drain(&mut scheduler).await;
            let reason = waited.expect("來源失敗能有限結束");
            assert_eq!(tracked, 0);
            assert_eq!(registered, 0);
            let expected = if failed {
                EndReason::SignalFailed("受控監聽錯誤".to_owned())
            } else {
                EndReason::StopSourceClosed
            };
            let batch = scheduler.into_batch(reason);
            assert_eq!(batch.reason, expected);
            assert_eq!(batch.entries.len(), 1);
            assert_unstarted(&batch.entries.as_slice()[0]);
            assert_eq!(batch.entries.as_slice()[0].original(), "/notes");
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn ordinary_completion_still_keeps_mixed_results_and_original_order() {
        let client = build_client(true).expect("client");
        let fixture = Fixture::start(0).await.expect("ready");
        let ok = fixture.url("/ok");
        let missing = fixture.url("/missing");
        let loop_url = fixture.url("/loop");
        let text = format!("\n{ok}\n \n/notes\n{ok}\n{missing}\n{loop_url}\n");
        let inputs = match prepare_inputs(text.as_str()) {
            Ok(inputs) => inputs,
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("五份材料：{error:?}；server={stopped:?}");
            }
        };
        let (mut scheduler, fixture) = make_scheduler(&client, inputs, fixture).await;
        let (_stop_tx, mut stop_rx) = mpsc::channel::<StopEvent>(1);
        let waited = timeout(Duration::from_secs(15), scheduler.run(&mut stop_rx)).await;
        let (batch, hits) = finish(scheduler, fixture, waited).await;
        assert_eq!(batch.reason, EndReason::Finished);
        assert_eq!(hits, 9);
        let indexes: Vec<usize> = batch
            .entries
            .iter()
            .map(|entry| entry.input_index)
            .collect();
        assert_eq!(indexes, vec![1, 3, 4, 5, 6]);
        let rows = batch.entries.as_slice();
        assert_eq!(rows[0].original(), ok.as_str());
        assert_eq!(rows[1].original(), "/notes");
        assert_eq!(rows[2].original(), ok.as_str());
        assert_eq!(rows[3].original(), missing.as_str());
        assert_eq!(rows[4].original(), loop_url.as_str());
        for index in [0, 2, 3] {
            match &rows[index].completion {
                Completion::Checked(Check {
                    outcome:
                        Outcome::Http {
                            status,
                            passed,
                            final_url: _,
                        },
                    original: _,
                    elapsed_ms: _,
                }) => {
                    let expected = if index == 3 { 404 } else { 200 };
                    assert_eq!(*status, expected);
                    assert_eq!(*passed, index != 3);
                }
                other => panic!("保留HTTP分類：{other:?}"),
            }
        }
        match &rows[1].completion {
            Completion::Checked(Check {
                outcome: Outcome::Input(_),
                original: _,
                elapsed_ms: _,
            }) => (),
            other => panic!("保留Input分類：{other:?}"),
        }
        match &rows[4].completion {
            Completion::Checked(Check {
                outcome: Outcome::Request(error),
                original: _,
                elapsed_ms: _,
            }) => assert!(error.is_redirect()),
            other => panic!("保留Request分類：{other:?}"),
        }
    }
}
