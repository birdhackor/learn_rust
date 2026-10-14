# 11.8 受控本機服務

axum 0.8.9、Tokio 1.53.3；path dependency重用11.7單一client。每份Fixture連續持有bind至server的listener，使用作業系統選出的loopback port，真GET /ready與body完成後才交回owner。沒有公共網站、固定port或sleep。

`Fixture::start(n)`準備n份一次使用的gate，input 0..n；`url`接本節以/開頭的固定path；`next_started`是無期限的通知等待；Fixture仍持有Sender，client Request錯誤不會讓它自動回None。caller使用`next_started_within(Duration::from_secs(5))`，以獨立5秒觀察預算取得 `Result<Option<usize>, Elapsed>`，與HTTP工作10秒不同。未確認進入時，`finish_client<T>`先abort＋await所有owned client handles、保留原Output，最後Fixture.shutdown；雙request分支收回兩者。所有相關assert/expect都延到這些資源收尾後。`release(input)`用Option.take接手送端，成功送入回true，不能反覆使用。`hits`只數工作handler進入，排除ready與extractor事先拒絕；它不是活躍數或成功數。

路由：/ok 200文字、/missing 404、/error 500、/redirect 307轉/ok、/loop 307轉自己、/held/{input}等待獨立gate後200文字；不存在／已使用gate 404，gate sender關閉後503。Path解析失敗先400，不進入handler。只延後headers，不提供streaming body。

`shutdown(self).await`先drop仍owned的gate senders，再通知graceful shutdown，最後await server。caller仍需另等client task；serverHandle的task結果與io工作結果分開。沒有Drop假裝await。Ready失敗亦先shutdown，再回診斷；source中需要的Mutex guard在await前以短block結束。

```bash
cargo run -p ch11-local-service --example basic --locked
cargo run -p ch11-local-service --locked
cargo run -p ch11-local-service --example closed_gate_solution --locked
cargo test -p ch11-local-service --locked
```

7 正常HTTP integration tests＋1 absent-entered故障路線＋1 CLI：status/body/finalURL、redirect上限、不同gate完成順序、handler ack後的request timeout、client取消後另server收尾、未放行shutdown得到503、同時活著的不同dynamicports、真binary完整輸出。timeout測試在真handler ack後pause Tokio clock，取得deadline結果後resume再cleanup；不以wall-clock耗時或std Instant當固定答案，也沒有聲稱它驗到body階段的deadline。

新增absent_entered使用/loop的真redirect Request錯誤，不走held handler。client結果通知受15秒期限保護；Result實際算出後才pause、測5秒entered Elapsed、resume。最後收回原Request錯誤及server總數6；不宣稱這是server abort故障的重現。
