# B2 起始版：保留原行與依序批次

對應15.8。請先讀正文新增的enumerate、trim、Input／Entry、Empty與guards契約。

`src/single.rs` 是B1 v8的完整單筆core，沒有新功能。`src/lib.rs` 的prepare_inputs／run_sequential兩body尚未實作，固定空Ok不符合本題Empty政策。普通target零tests，只證可編譯。

教材repo根目錄：

```bash
cargo check -p url-b2-starter --all-targets --locked
cargo test -p url-b2-starter --lib --locked
```

自行練習：保存完整Cargo.toml、src與checks/batch-tests.rs到repo外工作目錄。先把dev-dependency ch11-local-service的path換成你電腦上教材examples/ch11-async/local-service的絕對路徑，再跑：

```bash
cargo +1.99.0 check --all-targets
cargo +1.99.0 test --lib
```

接著把完整checks/batch-tests.rs追加到src/lib.rs，它已自帶test module；六項真失敗。先完成prepare_inputs使兩項通過，再完成run_sequential使六項通過。不改這份artifact保存自己的答案。

input_index為原始行0-based索引，略空白不重新編號；trim只判空，原文與duplicates保留。全部非空無效URL仍有item結果，零有效輸入回typed Empty。每項借同一client、各有B1期限；沒有整批timeout、CLIJSON或並行。held驗收用真事件、有限testguard與雙方收尾，不用sleep猜測。
