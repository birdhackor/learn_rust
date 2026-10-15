# A2 工作版

本階段承接已完成的 A1。`collection.rs` 保留八項 A1 驗收；`stream.rs`／`files.rs` 使用第9.4節已教的摘要、bytes 比較與前後觀察。它們都依賴本 package 的 `crate::collection` 型別。

要自己完成的三個函式在 `src/content.rs`：digest_candidates、refine_bucket、confirm_bucket。固定的空桶／空群組／Changed 只標示未完成，沒有觀察結果的意義。analyze 已接好這三份工作，還沒有最終報告或 CLI。

在自己的練習資料夾保存此 manifest 與完整 source，保留自己的 A1 collection 實作。先用 `cargo +1.99.0 check` 解析新依賴。八項 A1 測試通過只表示舊成果仍在。

`checks/content-tests.rs` 不是自動 test target；用編輯器把其中整個含條件屬性的 mod tests 加到 content.rs 末尾，才以 `cargo +1.99.0 test --locked` 驗收 A2。它包含可見的 fixture／短讀取／失敗來源，沒有隱藏的完整答案。

compare 的正常契約是完整相同回 true、不同回 false、不能完成回 FileIssue，失敗 path 必須是本次輸入裡的確切原生位置。重建移除失敗者後必須減少候選；違反 callback 契約是程式錯誤，完成版以 panic 阻止沒有進展的重試。一般檔案失敗則保存問題、繼續其他項目。
