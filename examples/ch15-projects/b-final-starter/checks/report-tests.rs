#[cfg(test)]
mod tests {
    use crate::batch::Batch;
    use crate::batch::Completion;
    use crate::batch::EndReason;
    use crate::batch::Entry;
    use crate::error::RunError;
    use crate::report::Report;
    use crate::report::Row;
    use crate::report::build_report;
    use crate::report::render;
    use crate::single::Check;
    use crate::single::InputIssue;
    use crate::single::Outcome;
    use std::time::Duration;
    use tokio::time::timeout;

    fn http(index: usize, passed: bool) -> Entry {
        Entry {
            input_index: index,
            completion: Completion::Checked(Check {
                original: "http://example.test/same".to_owned(),
                elapsed_ms: Some(12),
                outcome: Outcome::Http {
                    status: if passed { 200 } else { 404 },
                    passed,
                    final_url: "http://example.test/final".to_owned(),
                },
            }),
        }
    }

    fn sample(cancelled: bool, passed: bool) -> Report {
        Report {
            cancelled,
            items: vec![Row {
                input_index: 3,
                original: "http://example.test/same".to_owned(),
                kind: "http".to_owned(),
                passed,
                status: Some(if passed { 200 } else { 404 }),
                final_url: Some("http://example.test/final".to_owned()),
                elapsed_ms: Some(12),
                detail: None,
            }],
        }
    }

    // --8<-- [start:map-test]
    #[test]
    fn mapped_rows_keep_original_order_fields_and_absent_measurements() {
        let entries = vec![
            http(1, true),
            Entry {
                input_index: 3,
                completion: Completion::Checked(Check {
                    original: "ftp://host/item".to_owned(),
                    elapsed_ms: None,
                    outcome: Outcome::Input(InputIssue::Scheme("ftp".to_owned())),
                }),
            },
            http(4, false),
            Entry {
                input_index: 6,
                completion: Completion::Checked(Check {
                    original: "http://example.test/slow".to_owned(),
                    elapsed_ms: Some(1000),
                    outcome: Outcome::Timeout,
                }),
            },
            Entry {
                input_index: 8,
                completion: Completion::Cancelled {
                    original: "cancel-original".to_owned(),
                },
            },
            Entry {
                input_index: 9,
                completion: Completion::Unstarted {
                    original: "/notes".to_owned(),
                },
            },
        ];
        let report = build_report(Batch {
            entries,
            reason: EndReason::Requested,
        })
        .expect("使用者要求停止仍有報告");
        assert!(report.cancelled);
        let rows = report.items.as_slice();
        let indexes: Vec<usize> = rows.iter().map(|row| row.input_index).collect();
        assert_eq!(indexes, vec![1, 3, 4, 6, 8, 9]);
        let kinds: Vec<&str> = rows.iter().map(|row| row.kind.as_str()).collect();
        assert_eq!(
            kinds,
            vec!["http", "input", "http", "timeout", "cancelled", "unstarted"]
        );
        assert_eq!(rows[0].original.as_str(), rows[2].original.as_str());
        assert_eq!(rows[0].status, Some(200));
        assert!(rows[0].passed);
        assert_eq!(rows[2].status, Some(404));
        assert!(!rows[2].passed);
        assert_eq!(rows[0].elapsed_ms, Some(12));
        assert_eq!(rows[3].elapsed_ms, Some(1000));
        match &rows[0].final_url {
            Some(url) => assert_eq!(url.as_str(), "http://example.test/final"),
            None => panic!("HTTP有最終網址"),
        }
        assert_eq!(rows[1].original.as_str(), "ftp://host/item");
        match &rows[1].detail {
            Some(detail) => assert_eq!(detail.as_str(), "不接受 scheme：ftp"),
            None => panic!("輸入問題有診斷"),
        }
        for index in [1, 4, 5] {
            assert_eq!(rows[index].status, None);
            assert_eq!(rows[index].final_url, None);
            assert_eq!(rows[index].elapsed_ms, None);
        }
        assert_eq!(rows[4].original.as_str(), "cancel-original");
        assert_eq!(rows[5].original.as_str(), "/notes");
    }
    // --8<-- [end:map-test]

    #[test]
    fn source_failures_are_fatal_typed_errors_not_user_cancellation() {
        for (reason, failed) in [
            (EndReason::SignalFailed("受控來源失敗".to_owned()), true),
            (EndReason::StopSourceClosed, false),
        ] {
            match build_report(Batch {
                entries: Vec::new(),
                reason,
            }) {
                Err(RunError::Signal(message)) => {
                    assert!(failed);
                    assert_eq!(message.as_str(), "受控來源失敗");
                }
                Err(RunError::StopSourceClosed) => assert!(!failed),
                other => panic!("來源失敗不能交取消報告：{other:?}"),
            }
        }
    }

    // --8<-- [start:status-test]
    #[test]
    fn data_status_prioritizes_cancel_over_item_failure() {
        assert_eq!(sample(false, true).status(), 0);
        assert_eq!(sample(false, false).status(), 1);
        assert_eq!(sample(true, false).status(), 130);
        assert_eq!(sample(true, true).status(), 130);
    }
    // --8<-- [end:status-test]

    #[test]
    fn both_formats_preserve_owned_model_and_escape_text_rows() {
        let mut report = sample(false, false);
        let row = &mut report.items.as_mut_slice()[0];
        // 這是手工codec材料，不是假稱一次HTTP請求產生此診斷。
        row.original = "含\"引號\"的原文".to_owned();
        row.detail = Some("第一行\n第二行".to_owned());
        let json = render(&report, true).expect("模型可編碼");
        let recovered = serde_json::from_str::<Report>(json.as_str()).expect("整份JSON符合模型");
        assert_eq!(recovered, report);
        let text = render(&report, false).expect("文字欄位的JSON引用可編碼");
        assert_eq!(text.lines().fold(0, |count: usize, _line| count + 1), 2);
        assert!(text.contains("kind=http"));
        assert!(text.contains("original=\"含\\\"引號\\\"的原文\""));
        assert!(text.contains("detail=\"第一行\\n第二行\""));
        assert_eq!(render(&report, false).expect("同一模型"), text);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn task_failure_keeps_original_index_without_fake_http_or_timing() {
        let mut task: tokio::task::JoinHandle<()> = tokio::spawn(async {
            panic!("受控task故障");
        });
        let joined = timeout(Duration::from_secs(5), &mut task).await;
        let error = match joined {
            Ok(Err(error)) => error,
            Ok(Ok(())) => panic!("人工task應故障"),
            Err(error) => {
                task.abort();
                let finished = task.await;
                panic!("故障觀察到期：{error}；收尾={finished:?}");
            }
        };
        let report = build_report(Batch {
            entries: vec![Entry {
                input_index: 7,
                completion: Completion::TaskFailure {
                    original: "raw-original".to_owned(),
                    error,
                },
            }],
            reason: EndReason::Finished,
        })
        .expect("task故障是項目結果");
        let row = &report.items.as_slice()[0];
        assert_eq!(row.input_index, 7);
        assert_eq!(row.original.as_str(), "raw-original");
        assert_eq!(row.kind.as_str(), "task_failure");
        assert_eq!(row.status, None);
        assert_eq!(row.elapsed_ms, None);
        match &row.detail {
            Some(detail) => assert!(!detail.is_empty()),
            None => panic!("保留task原因"),
        }
    }
}
