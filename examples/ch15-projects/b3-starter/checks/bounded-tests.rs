#[cfg(test)]
mod tests {
    use crate::Completion;
    use crate::ConfigError;
    use crate::Entry;
    use crate::Scheduler;
    use crate::default_jobs;
    use crate::input::Input;
    use crate::input::prepare_inputs;
    use crate::order_entries;
    use crate::run_bounded;
    use crate::single::Check;
    use crate::single::InputIssue;
    use crate::single::Outcome;
    use crate::single::build_client;
    use crate::single::default_limit;
    use ch11_local_service::Fixture;
    use reqwest::Client;
    use std::collections::HashMap;
    use std::time::Duration;
    use tokio::sync::mpsc;
    use tokio::task::JoinSet;
    use tokio::time::timeout;

    fn assert_http(entry: &Entry, status: u16, passed: bool) {
        match &entry.completion {
            Completion::Checked(Check {
                outcome:
                    Outcome::Http {
                        status: actual,
                        passed: actual_passed,
                        final_url: _,
                    },
                original: _,
                elapsed_ms: _,
            }) => {
                assert_eq!(*actual, status);
                assert_eq!(*actual_passed, passed);
            }
            other => panic!("應有 HTTP 結果：{other:?}"),
        }
    }

    // --8<-- [start:drain]
    async fn drain(scheduler: &mut Scheduler) {
        scheduler.tasks.abort_all();
        while let Some(_result) = scheduler.tasks.join_next_with_id().await {}
    }

    async fn close(scheduler: &mut Scheduler, fixture: Fixture) -> Result<usize, String> {
        drain(scheduler).await;
        fixture.shutdown().await
    }
    // --8<-- [end:drain]

    // --8<-- [start:guards]
    async fn entered(mut fixture: Fixture, scheduler: &mut Scheduler) -> (Fixture, usize) {
        let observed = timeout(Duration::from_secs(5), fixture.next_started()).await;
        match observed {
            Ok(Some(index)) => (fixture, index),
            other => {
                let stopped = close(scheduler, fixture).await;
                panic!("沒有取得 handler 進入通知：{other:?}；收尾={stopped:?}");
            }
        }
    }

    async fn take(
        fixture: Fixture,
        scheduler: &mut Scheduler,
        expected: usize,
    ) -> (Fixture, Entry) {
        let observed = timeout(Duration::from_secs(5), scheduler.next()).await;
        let entry = match observed {
            Ok(Some(entry)) => entry,
            other => {
                let stopped = close(scheduler, fixture).await;
                panic!("應收取 index={expected}：{other:?}；收尾={stopped:?}");
            }
        };
        if entry.input_index != expected {
            let stopped = close(scheduler, fixture).await;
            panic!("應收取 index={expected}：{entry:?}；收尾={stopped:?}");
        }
        (fixture, entry)
    }

    async fn release(mut fixture: Fixture, scheduler: &mut Scheduler, index: usize) -> Fixture {
        if fixture.release(index).await {
            fixture
        } else {
            let stopped = close(scheduler, fixture).await;
            panic!("gate {index} 未放行；收尾={stopped:?}");
        }
    }
    // --8<-- [end:guards]

    #[tokio::test(flavor = "current_thread")]
    async fn zero_jobs_is_rejected_before_any_request() {
        let client = build_client(true).expect("本機 client");
        let fixture = Fixture::start(0).await.expect("ready");
        let inputs = vec![Input {
            input_index: 0,
            original: fixture.url("/ok"),
        }];
        let result = run_bounded(&client, inputs, 0, default_limit()).await;
        let stopped = fixture.shutdown().await;
        assert_eq!(stopped, Ok(0));
        assert_eq!(default_jobs(), 8);
        match result {
            Err(ConfigError::ZeroJobs) => (),
            other => panic!("零 jobs 必須拒絕：{other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn empty_owned_list_is_rejected() {
        let client = build_client(true).expect("本機 client");
        let fixture = Fixture::start(0).await.expect("ready");
        let result = run_bounded(&client, Vec::new(), 2, default_limit()).await;
        let stopped = fixture.shutdown().await;
        assert_eq!(stopped, Ok(0));
        match result {
            Err(ConfigError::Empty) => (),
            other => panic!("空有效清單必須拒絕：{other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn invalid_items_are_results_and_do_not_make_the_list_empty() {
        let client = build_client(true).expect("本機 client");
        let inputs = prepare_inputs("/notes\nfile:///tmp/report.txt\n").expect("兩份非空行");
        let fixture = Fixture::start(0).await.expect("ready");
        let result = run_bounded(&client, inputs, 2, default_limit()).await;
        let stopped = fixture.shutdown().await;
        assert_eq!(stopped, Ok(0));
        let rows = result.expect("非空清單");
        let rows = rows.as_slice();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].input_index, 0);
        assert_eq!(rows[1].input_index, 1);
        assert_eq!(rows[0].original(), "/notes");
        assert_eq!(rows[1].original(), "file:///tmp/report.txt");
        match &rows[0].completion {
            Completion::Checked(check) => {
                assert_eq!(check.elapsed_ms, None);
                match &check.outcome {
                    Outcome::Input(InputIssue::Url(_)) => (),
                    other => panic!("相對路徑解析問題：{other:?}"),
                }
            }
            other => panic!("應有單筆結果：{other:?}"),
        }
        match &rows[1].completion {
            Completion::Checked(check) => {
                assert_eq!(check.elapsed_ms, None);
                match &check.outcome {
                    Outcome::Input(InputIssue::Scheme(scheme)) => {
                        assert_eq!(scheme.as_str(), "file")
                    }
                    other => panic!("scheme 問題：{other:?}"),
                }
            }
            other => panic!("應有單筆結果：{other:?}"),
        }
    }

    // --8<-- [start:mixed-test]
    #[tokio::test(flavor = "current_thread")]
    async fn mixed_items_continue_and_return_original_input_order() {
        let client = build_client(true).expect("本機 client");
        let fixture = Fixture::start(0).await.expect("ready");
        let ok = fixture.url("/ok");
        let missing = fixture.url("/missing");
        let loop_url = fixture.url("/loop");
        let text = format!("\n{ok}\n \n/notes\n{ok}\n{missing}\n{loop_url}\n");
        let inputs = match prepare_inputs(text.as_str()) {
            Ok(inputs) => inputs,
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("材料應有五筆：{error:?}；收尾={stopped:?}");
            }
        };
        let result = run_bounded(&client, inputs, 2, default_limit()).await;
        let stopped = fixture.shutdown().await;
        assert_eq!(stopped, Ok(9));
        let rows = result.expect("item 失敗仍保留整批結果");
        let indexes: Vec<usize> = rows.iter().map(|entry| entry.input_index).collect();
        assert_eq!(indexes, vec![1, 3, 4, 5, 6]);
        let rows = rows.as_slice();
        assert_eq!(rows[0].original(), ok.as_str());
        assert_eq!(rows[1].original(), "/notes");
        assert_eq!(rows[2].original(), ok.as_str());
        assert_http(&rows[0], 200, true);
        match &rows[1].completion {
            Completion::Checked(Check {
                outcome: Outcome::Input(InputIssue::Url(_)),
                original: _,
                elapsed_ms: _,
            }) => (),
            other => panic!("輸入錯誤仍保存：{other:?}"),
        }
        assert_http(&rows[2], 200, true);
        assert_http(&rows[3], 404, false);
        match &rows[4].completion {
            Completion::Checked(Check {
                outcome: Outcome::Request(error),
                original: _,
                elapsed_ms: _,
            }) => assert!(error.is_redirect()),
            other => panic!("redirect 政策錯誤仍保存：{other:?}"),
        }
    }
    // --8<-- [end:mixed-test]

    // --8<-- [start:held-test]
    #[tokio::test(flavor = "current_thread")]
    async fn collect_one_before_refilling_and_sort_only_after_collection() {
        let client = build_client(true).expect("本機 client");
        let fixture = Fixture::start(3).await.expect("ready");
        let first = fixture.url("/held/0");
        let second = fixture.url("/held/1");
        let third = fixture.url("/held/2");
        let inputs = vec![
            Input {
                input_index: 0,
                original: first.clone(),
            },
            Input {
                input_index: 1,
                original: second.clone(),
            },
            Input {
                input_index: 2,
                original: third.clone(),
            },
        ];
        let mut scheduler = match Scheduler::new(&client, inputs, 2, default_limit()) {
            Ok(scheduler) => scheduler,
            Err(error) => {
                let stopped = fixture.shutdown().await;
                panic!("合法設定應建立 scheduler：{error:?}；收尾={stopped:?}");
            }
        };
        let (fixture, a) = entered(fixture, &mut scheduler).await;
        let (fixture, b) = entered(fixture, &mut scheduler).await;
        let first_pair = (a == 0 && b == 1) || (a == 1 && b == 0);
        let first_tracked = scheduler.tracked();
        let first_registered = scheduler.inputs.len();
        if !first_pair || first_tracked != 2 || first_registered != 2 {
            let stopped = close(&mut scheduler, fixture).await;
            panic!(
                "初始只能保有 0/1 兩份完整登記：{a}/{b}/{first_tracked}/{first_registered}；收尾={stopped:?}"
            );
        }
        let fixture = release(fixture, &mut scheduler, 1).await;
        let (fixture, second_entry) = take(fixture, &mut scheduler, 1).await;
        let (fixture, c) = entered(fixture, &mut scheduler).await;
        let refilled = scheduler.tracked();
        if c != 2 || refilled != 2 || scheduler.inputs.len() != 2 {
            let stopped = close(&mut scheduler, fixture).await;
            panic!("收取 1 後才補入 2：{c}/{refilled}；收尾={stopped:?}");
        }
        let fixture = release(fixture, &mut scheduler, 2).await;
        let (fixture, third_entry) = take(fixture, &mut scheduler, 2).await;
        let fixture = release(fixture, &mut scheduler, 0).await;
        let (fixture, first_entry) = take(fixture, &mut scheduler, 0).await;
        let exhausted = timeout(Duration::from_secs(5), scheduler.next()).await;
        let stopped = close(&mut scheduler, fixture).await;
        assert_eq!(stopped, Ok(3));
        match exhausted {
            Ok(None) => (),
            other => panic!("全收取後才為 None：{other:?}"),
        }
        let mut rows = vec![second_entry, third_entry, first_entry];
        let completion_order: Vec<usize> = rows.iter().map(|entry| entry.input_index).collect();
        assert_eq!(completion_order, vec![1, 2, 0]);
        order_entries(rows.as_mut_slice());
        let input_order: Vec<usize> = rows.iter().map(|entry| entry.input_index).collect();
        assert_eq!(input_order, vec![0, 1, 2]);
        let rows = rows.as_slice();
        assert_eq!(rows[0].original(), first.as_str());
        assert_eq!(rows[1].original(), second.as_str());
        assert_eq!(rows[2].original(), third.as_str());
        for row in rows {
            assert_http(row, 200, true);
        }
    }
    // --8<-- [end:held-test]

    // --8<-- [start:task-lab]
    fn task_lab(client: &Client, pending: Vec<Input>, jobs: usize) -> Scheduler {
        Scheduler {
            pending: pending.into_iter(),
            tasks: JoinSet::new(),
            inputs: HashMap::new(),
            client: client.clone(),
            jobs,
            limit: default_limit(),
        }
    }

    fn lab_check(original: String) -> Check {
        Check {
            original,
            elapsed_ms: None,
            outcome: Outcome::Timeout,
        }
    }

    async fn signal(scheduler: &mut Scheduler, receiver: &mut mpsc::Receiver<()>) {
        let observed = timeout(Duration::from_secs(5), receiver.recv()).await;
        if let Ok(Some(())) = observed {
            return;
        }
        drain(scheduler).await;
        panic!("合成 task 通知未到：{observed:?}");
    }

    async fn lab_take(scheduler: &mut Scheduler) -> Entry {
        let observed = timeout(Duration::from_secs(5), scheduler.next()).await;
        match observed {
            Ok(Some(entry)) => entry,
            other => {
                drain(scheduler).await;
                panic!("合成 task 無收取結果：{other:?}");
            }
        }
    }
    // --8<-- [end:task-lab]

    // --8<-- [start:fault-test]
    #[tokio::test(flavor = "current_thread")]
    async fn panic_and_abort_keep_the_registered_index_and_original() {
        let client = build_client(true).expect("本機 client");
        let mut scheduler = task_lab(&client, Vec::new(), 2);
        let (entered_tx, mut entered_rx) = mpsc::channel::<()>(2);
        let (release_tx, mut release_rx) = mpsc::channel::<()>(1);
        let first_entered = entered_tx.clone();
        let first_abort = scheduler.tasks.spawn(async move {
            let _sent = first_entered.send(()).await;
            let _released = release_rx.recv().await;
            panic!("受控的合成 task panic");
        });
        scheduler.inputs.insert(
            first_abort.id(),
            Input {
                input_index: 4,
                original: "panic-original".to_owned(),
            },
        );
        let (_held_tx, mut held_rx) = mpsc::channel::<()>(1);
        let second_abort = scheduler.tasks.spawn(async move {
            let _sent = entered_tx.send(()).await;
            let _released = held_rx.recv().await;
            lab_check("abort-original".to_owned())
        });
        scheduler.inputs.insert(
            second_abort.id(),
            Input {
                input_index: 9,
                original: "abort-original".to_owned(),
            },
        );
        signal(&mut scheduler, &mut entered_rx).await;
        signal(&mut scheduler, &mut entered_rx).await;
        if let Err(error) = release_tx.send(()).await {
            drain(&mut scheduler).await;
            panic!("合成 panic 放行失敗：{error:?}");
        }
        let first = lab_take(&mut scheduler).await;
        second_abort.abort();
        let second = lab_take(&mut scheduler).await;
        drain(&mut scheduler).await;
        assert_eq!(first.input_index, 4);
        assert_eq!(first.original(), "panic-original");
        assert_eq!(second.input_index, 9);
        assert_eq!(second.original(), "abort-original");
        match &first.completion {
            Completion::TaskFailure { error, original: _ } => assert!(error.is_panic()),
            other => panic!("panic 必須是 task 層錯誤：{other:?}"),
        }
        match &second.completion {
            Completion::TaskFailure { error, original: _ } => assert!(error.is_cancelled()),
            other => panic!("abort 必須是 task 層錯誤：{other:?}"),
        }
    }
    // --8<-- [end:fault-test]

    // --8<-- [start:completed-test]
    #[tokio::test(flavor = "current_thread")]
    async fn completed_but_uncollected_task_still_occupies_the_slot() {
        let client = build_client(true).expect("本機 client");
        let pending = vec![Input {
            input_index: 1,
            original: "/pending".to_owned(),
        }];
        let mut scheduler = task_lab(&client, pending, 1);
        let (done_tx, mut done_rx) = mpsc::channel::<()>(1);
        let abort = scheduler.tasks.spawn(async move {
            // 空且容量為 1；這次 send 可完成，之後直到 return 都沒有 await。
            let _sent = done_tx.send(()).await;
            lab_check("finished".to_owned())
        });
        scheduler.inputs.insert(
            abort.id(),
            Input {
                input_index: 0,
                original: "finished".to_owned(),
            },
        );
        signal(&mut scheduler, &mut done_rx).await;
        let before = scheduler.tracked();
        scheduler.fill();
        let after = scheduler.tracked();
        let registered = scheduler.inputs.len();
        drain(&mut scheduler).await;
        assert_eq!(before, 1);
        assert_eq!(after, 1);
        assert_eq!(registered, 1);
    }
    // --8<-- [end:completed-test]
}
