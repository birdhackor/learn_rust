# A1 工作版

這是第 15.2 節的獨立起始 library，尚未完成掃描。

- `src/lib.rs` 宣告 `collection` module。
- `src/collection.rs` 的 `observe`、`validate_root`、`collect_entries` 是明示的待做位置；root 固定回傳 `NotDirectory`，沒有真的檢查 root。
- `checks/collection-tests.rs` 是要手動加入的驗收材料。Cargo 不會自動把 `checks/` 當成 integration tests；請用編輯器把其中整個 `#[cfg(test)] mod tests` 加到 `src/collection.rs` 末尾。

在自己的練習資料夾保存本 manifest 與上述 source，不保留 Cargo new 產生的 `src/main.rs`。先執行 `cargo +1.99.0 check` 解析依賴；加入驗收材料後，以 `cargo +1.99.0 test --locked` 看到行為失敗，再自行實作。起始版若沒有加入驗收，零項 tests 不代表功能完成。

本階段只收集一般檔案與項目問題，還沒有內容判定、最終報告或完整 CLI。參考答案在同層 `a1-complete`，只包含 A1 成果。
