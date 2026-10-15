pub mod single;

use reqwest::Client;
use std::time::Duration;

// --8<-- [start:models]
#[derive(Debug)]
pub struct Input {
    pub input_index: usize,
    pub original: String,
}

#[derive(Debug)]
pub struct Entry {
    pub input_index: usize,
    pub check: single::Check,
}

#[derive(Debug)]
pub enum InputError {
    Empty,
}
// --8<-- [end:models]

// --8<-- [start:prepare]
pub fn prepare_inputs(text: &str) -> Result<Vec<Input>, InputError> {
    let mut inputs = Vec::new();
    for (input_index, line) in text.lines().enumerate() {
        if !line.trim().is_empty() {
            inputs.push(Input {
                input_index,
                original: line.to_owned(),
            });
        }
    }
    if inputs.is_empty() {
        Err(InputError::Empty)
    } else {
        Ok(inputs)
    }
}
// --8<-- [end:prepare]

// --8<-- [start:sequential]
pub async fn run_sequential(
    client: &Client,
    inputs: Vec<Input>,
    limit: Duration,
) -> Result<Vec<Entry>, InputError> {
    if inputs.is_empty() {
        return Err(InputError::Empty);
    }
    let mut entries = Vec::new();
    for input in inputs {
        let input_index = input.input_index;
        let check = single::check(client, input.original, limit).await;
        entries.push(Entry { input_index, check });
    }
    Ok(entries)
}
// --8<-- [end:sequential]

#[cfg(test)]
mod tests {
    use crate::Entry;
    use crate::InputError;
    use crate::prepare_inputs;
    use crate::run_sequential;
    use crate::single::InputIssue;
    use crate::single::Outcome;
    use crate::single::build_client;
    use crate::single::default_limit;
    use ch11_local_service::Fixture;
    use std::time::Duration;
    use tokio::task::JoinHandle;
    use tokio::time::timeout;

    fn assert_http(entry: &Entry, status: u16, passed: bool) {
        match &entry.check.outcome {
            Outcome::Http {
                status: actual_status,
                passed: actual_passed,
                final_url: _,
            } => {
                assert_eq!(*actual_status, status);
                assert_eq!(*actual_passed, passed);
            }
            other => panic!("應有HTTP結果：{other:?}"),
        }
    }

    // --8<-- [start:input-test]
    #[test]
    fn owned_lines_keep_original_indexes_spaces_and_duplicates() {
        let inputs = {
            let text = " \r\n first \r\n\r\nsecond\n first \n".to_owned();
            prepare_inputs(text.as_str()).expect("有三份非空白輸入")
        };
        let inputs = inputs.as_slice();
        assert_eq!(inputs.len(), 3);
        assert_eq!(inputs[0].input_index, 1);
        assert_eq!(inputs[1].input_index, 3);
        assert_eq!(inputs[2].input_index, 4);
        assert_eq!(inputs[0].original.as_str(), " first ");
        assert_eq!(inputs[1].original.as_str(), "second");
        assert_eq!(inputs[2].original.as_str(), " first ");
        let lone_cr = prepare_inputs("a\rb").expect("單獨CR沒有分行");
        assert_eq!(lone_cr.as_slice()[0].original.as_str(), "a\rb");
    }
    // --8<-- [end:input-test]

    #[test]
    fn no_nonblank_lines_is_a_configuration_error() {
        // 第三份literal第二行是U+3000全形空白。
        for text in ["", "\n", " \r\n　\n"] {
            match prepare_inputs(text) {
                Err(InputError::Empty) => (),
                other => panic!("空有效清單必須拒絕：{other:?}"),
            }
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn runner_also_refuses_an_empty_owned_list() {
        let fixture = Fixture::start(0).await.expect("ready");
        let client = build_client(true).expect("本機client");
        let result = run_sequential(&client, Vec::new(), default_limit()).await;
        let hits = fixture.shutdown().await;
        assert_eq!(hits, Ok(0));
        match result {
            Err(InputError::Empty) => (),
            other => panic!("runner也必須拒絕零項：{other:?}"),
        }
    }

    #[tokio::test(flavor = "current_thread")]
    async fn nonempty_invalid_only_list_still_returns_each_item() {
        let fixture = Fixture::start(0).await.expect("ready");
        let client = build_client(true).expect("本機client");
        let inputs = prepare_inputs("/notes\nfile:///tmp/report.txt\n")
            .expect("兩份非空白行，還沒有判URL有效性");
        let result = run_sequential(&client, inputs, default_limit()).await;
        let hits = fixture.shutdown().await;
        assert_eq!(hits, Ok(0));
        let rows = result.expect("item失敗不等於Empty");
        let rows = rows.as_slice();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].input_index, 0);
        assert_eq!(rows[1].input_index, 1);
        assert_eq!(rows[0].check.original.as_str(), "/notes");
        assert_eq!(rows[1].check.original.as_str(), "file:///tmp/report.txt");
        assert_eq!(rows[0].check.elapsed_ms, None);
        assert_eq!(rows[1].check.elapsed_ms, None);
        match &rows[0].check.outcome {
            Outcome::Input(InputIssue::Url(_)) => (),
            other => panic!("相對路徑是URL解析問題：{other:?}"),
        }
        match &rows[1].check.outcome {
            Outcome::Input(InputIssue::Scheme(scheme)) => assert_eq!(scheme.as_str(), "file"),
            other => panic!("file是scheme問題：{other:?}"),
        }
    }

    // --8<-- [start:mixed-test]
    #[tokio::test(flavor = "current_thread")]
    async fn mixed_results_continue_and_duplicate_urls_remain_separate() {
        let fixture = Fixture::start(0).await.expect("ready");
        let client = build_client(true).expect("本機client");
        let ok = fixture.url("/ok");
        let missing = fixture.url("/missing");
        let loop_url = fixture.url("/loop");
        let text = format!("\n{ok}\n \n/notes\n{ok}\n{missing}\n{loop_url}\n");
        let inputs = prepare_inputs(text.as_str()).expect("五份非空白行");
        let result = run_sequential(&client, inputs, default_limit()).await;
        let hits = fixture.shutdown().await;
        assert_eq!(hits, Ok(9));
        let rows = result.expect("item失敗仍有整份批次結果");
        let indexes: Vec<usize> = rows.iter().map(|entry| entry.input_index).collect();
        assert_eq!(indexes, vec![1, 3, 4, 5, 6]);
        let rows = rows.as_slice();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].check.original.as_str(), ok.as_str());
        assert_eq!(rows[1].check.original.as_str(), "/notes");
        assert_eq!(rows[2].check.original.as_str(), ok.as_str());
        assert_http(&rows[0], 200, true);
        match &rows[1].check.outcome {
            Outcome::Input(InputIssue::Url(_)) => (),
            other => panic!("中間的輸入問題仍保存：{other:?}"),
        }
        assert_http(&rows[2], 200, true);
        assert_http(&rows[3], 404, false);
        match &rows[4].check.outcome {
            Outcome::Request(error) => assert!(error.is_redirect()),
            other => panic!("redirect政策問題仍保存：{other:?}"),
        }
    }
    // --8<-- [end:mixed-test]

    // --8<-- [start:gate-helpers]
    type BatchTask = JoinHandle<Result<Vec<Entry>, InputError>>;

    async fn observe_started(
        mut fixture: Fixture,
        batch: BatchTask,
        expected: usize,
    ) -> (Fixture, BatchTask) {
        let observed = timeout(Duration::from_secs(5), fixture.next_started()).await;
        let correct = match &observed {
            Ok(Some(index)) => *index == expected,
            _ => false,
        };
        if correct {
            return (fixture, batch);
        }
        batch.abort();
        let client_finished = batch.await;
        let server_finished = fixture.shutdown().await;
        panic!(
            "應進入held{expected}：{observed:?}；client={client_finished:?}；server={server_finished:?}"
        );
    }

    async fn finish_batch(fixture: Fixture, mut batch: BatchTask) -> (Vec<Entry>, usize) {
        let waited = timeout(Duration::from_secs(5), &mut batch).await;
        let result = match waited {
            Ok(result) => result,
            Err(error) => {
                batch.abort();
                let client_finished = batch.await;
                let server_finished = fixture.shutdown().await;
                panic!(
                    "放行後仍未取得結果：{error}；client={client_finished:?}；server={server_finished:?}"
                );
            }
        };
        let hits = fixture.shutdown().await.expect("server完成收尾");
        let rows = result.expect("task完成").expect("非空輸入有結果");
        (rows, hits)
    }
    // --8<-- [end:gate-helpers]

    // --8<-- [start:gate-test]
    #[tokio::test(flavor = "current_thread")]
    async fn held_inputs_wait_and_results_keep_the_two_input_positions() {
        let fixture = Fixture::start(2).await.expect("ready");
        let client = build_client(true).expect("本機client");
        let first = fixture.url("/held/0");
        let second = fixture.url("/held/1");
        let text = format!("{first}\n{second}\n");
        let inputs = prepare_inputs(text.as_str()).expect("兩份輸入");
        let batch =
            tokio::spawn(async move { run_sequential(&client, inputs, default_limit()).await });
        let (mut fixture, batch) = observe_started(fixture, batch, 0).await;
        let first_hits = fixture.hits();
        let was_pending = !batch.is_finished();
        let first_released = fixture.release(0).await;
        let (mut fixture, batch) = observe_started(fixture, batch, 1).await;
        let second_released = fixture.release(1).await;
        let (rows, hits) = finish_batch(fixture, batch).await;
        assert_eq!(first_hits, 1);
        assert!(was_pending);
        assert!(first_released);
        assert!(second_released);
        assert_eq!(hits, 2);
        let rows = rows.as_slice();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].input_index, 0);
        assert_eq!(rows[1].input_index, 1);
        assert_eq!(rows[0].check.original.as_str(), first.as_str());
        assert_eq!(rows[1].check.original.as_str(), second.as_str());
        assert_http(&rows[0], 200, true);
        assert_http(&rows[1], 200, true);
    }
    // --8<-- [end:gate-test]
}
