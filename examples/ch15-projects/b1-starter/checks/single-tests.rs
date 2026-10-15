#[cfg(test)]
mod tests {
    use crate::Check;
    use crate::InputIssue;
    use crate::Outcome;
    use crate::build_client;
    use crate::check;
    use crate::default_limit;
    use ch11_local_service::Fixture;
    use std::time::Duration;
    use tokio::task::JoinHandle;
    use tokio::time::timeout;

    fn assert_http(record: &Check, status: u16, passed: bool, final_url: &str) {
        match &record.outcome {
            Outcome::Http {
                final_url: actual_url,
                status: actual_status,
                passed: actual_passed,
            } => {
                assert_eq!(actual_url.as_str(), final_url);
                assert_eq!(*actual_status, status);
                assert_eq!(*actual_passed, passed);
            }
            other => panic!("應有 HTTP 結果：{other:?}"),
        }
        match &record.elapsed_ms {
            Some(_) => (),
            None => panic!("真正開始的請求須有量測結果"),
        }
    }

    // --8<-- [start:observation-helper]
    async fn held_request(limit: Duration) -> (Fixture, JoinHandle<Check>) {
        let mut fixture = Fixture::start(1).await.expect("ready 確認真本機服務");
        let client = build_client(true).expect("能建立本機 client");
        let original = fixture.url("/held/0");
        let request = tokio::spawn(async move { check(&client, original, limit).await });
        let observed = timeout(Duration::from_secs(5), fixture.next_started()).await;
        match observed {
            Ok(Some(0)) => (fixture, request),
            other => {
                request.abort();
                let client_finished = request.await;
                let server_finished = fixture.shutdown().await;
                panic!(
                    "沒有觀察到 held0 進入：{other:?}；client={client_finished:?}；server={server_finished:?}"
                );
            }
        }
    }
    // --8<-- [end:observation-helper]

    // --8<-- [start:input-test]
    #[tokio::test(flavor = "current_thread")]
    async fn invalid_input_keeps_original_and_does_not_start_a_request() {
        let fixture = Fixture::start(0).await.expect("ready 確認真本機服務");
        let client = build_client(true).expect("能建立本機 client");
        let malformed = check(&client, "/notes".to_owned(), default_limit()).await;
        let scheme = check(&client, "file:///tmp/notes.txt".to_owned(), default_limit()).await;
        let final_hits = fixture.shutdown().await;
        assert_eq!(final_hits, Ok(0));
        assert_eq!(malformed.original.as_str(), "/notes");
        assert_eq!(malformed.elapsed_ms, None);
        match malformed.outcome {
            Outcome::Input(InputIssue::Url(_)) => (),
            other => panic!("相對路徑應在解析處拒絕：{other:?}"),
        }
        assert_eq!(scheme.original.as_str(), "file:///tmp/notes.txt");
        assert_eq!(scheme.elapsed_ms, None);
        match scheme.outcome {
            Outcome::Input(InputIssue::Scheme(scheme)) => assert_eq!(scheme.as_str(), "file"),
            other => panic!("已解析但不支援的 scheme：{other:?}"),
        }
    }
    // --8<-- [end:input-test]

    // --8<-- [start:http-test]
    #[tokio::test(flavor = "current_thread")]
    async fn one_shared_client_keeps_success_and_http_not_passed_as_http_results() {
        let fixture = Fixture::start(0).await.expect("ready 確認真本機服務");
        let client = build_client(true).expect("能建立本機 client");
        let mut observations = Vec::new();
        for spec in [
            ("/ok", 200, true),
            ("/missing", 404, false),
            ("/error", 500, false),
        ] {
            let original = fixture.url(spec.0);
            let record = check(&client, original, default_limit()).await;
            observations.push((record, spec.1, spec.2));
        }
        let final_hits = fixture.shutdown().await;
        assert_eq!(final_hits, Ok(3));
        for (record, status, passed) in observations {
            assert_http(&record, status, passed, record.original.as_str());
        }
    }
    // --8<-- [end:http-test]

    // --8<-- [start:redirect-test]
    #[tokio::test(flavor = "current_thread")]
    async fn redirect_keeps_original_and_returns_the_final_headers_url() {
        let fixture = Fixture::start(0).await.expect("ready 確認真本機服務");
        let client = build_client(true).expect("能建立本機 client");
        let original = fixture.url("/redirect");
        let expected_original = original.as_str().to_owned();
        let expected_final = fixture.url("/ok");
        let record = check(&client, original, default_limit()).await;
        let final_hits = fixture.shutdown().await;
        assert_eq!(final_hits, Ok(2));
        assert_eq!(record.original, expected_original);
        assert_http(&record, 200, true, expected_final.as_str());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn redirect_limit_is_a_request_problem_without_a_final_http_result() {
        let fixture = Fixture::start(0).await.expect("ready 確認真本機服務");
        let client = build_client(true).expect("能建立本機 client");
        let original = fixture.url("/loop");
        let record = check(&client, original, default_limit()).await;
        let final_hits = fixture.shutdown().await;
        assert_eq!(final_hits, Ok(6));
        match &record.outcome {
            Outcome::Request(error) => assert!(error.is_redirect()),
            other => panic!("第六次轉向應由政策拒絕：{other:?}"),
        }
        match &record.elapsed_ms {
            Some(_) => (),
            None => panic!("已開始請求須保留量測"),
        }
    }
    // --8<-- [end:redirect-test]

    // --8<-- [start:gate-test]
    #[tokio::test(flavor = "current_thread")]
    async fn a_held_request_waits_for_release_and_then_returns_http() {
        let (mut fixture, request) = held_request(default_limit()).await;
        let was_pending = !request.is_finished();
        let expected_final = fixture.url("/held/0");
        let released = fixture.release(0).await;
        let result = request.await;
        let final_hits = fixture.shutdown().await;
        assert!(was_pending);
        assert!(released);
        assert_eq!(final_hits, Ok(1));
        let record = result.expect("request task 正常交回單筆結果");
        assert_http(&record, 200, true, expected_final.as_str());
    }
    // --8<-- [end:gate-test]

    // --8<-- [start:timeout-test]
    #[tokio::test(flavor = "current_thread")]
    async fn whole_headers_work_expires_after_real_handler_acknowledgement() {
        let (fixture, mut request) = held_request(default_limit()).await;
        let was_pending = !request.is_finished();
        tokio::time::pause();
        let waited = timeout(Duration::from_secs(20), &mut request).await;
        tokio::time::resume();
        let result = match waited {
            Ok(result) => result,
            Err(error) => {
                request.abort();
                let client_finished = request.await;
                let server_finished = fixture.shutdown().await;
                panic!(
                    "觀察期限已到，工作仍未交回：{error}；client={client_finished:?}；server={server_finished:?}"
                );
            }
        };
        let final_hits = fixture.shutdown().await;
        assert!(was_pending);
        assert_eq!(final_hits, Ok(1));
        let record = result.expect("期限是正常task Output，不是task失敗");
        match &record.outcome {
            Outcome::Timeout => (),
            other => panic!("headers仍受阻，應是Timeout：{other:?}"),
        }
        match &record.elapsed_ms {
            Some(_) => (),
            None => panic!("request已開始，真時間可量但不固定10秒"),
        }
    }
    // --8<-- [end:timeout-test]
}
