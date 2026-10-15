# A2 完成快照

這份 library 承接 A1，完成大小篩選、BLAKE3 候選、真正 bytes 分組與項目失敗後的有限重建。沒有 A3 的文字／JSON 報告、指標或 A4 完整 CLI。

從教材 repo 根目錄：

```bash
cargo test -p duplicate-a2 --locked
```

Unix 上共有八項 A1 與十二項 A2 驗收，其他平台不編入三項 Unix fixtures。A2 驗收與工作版 checks/content-tests.rs 完全相同，真正編在 content.rs 末尾。

真實六檔材料的示範入口固定如下：

```bash
cd examples/ch15-projects/a2-complete
cargo run --example a2_groups --locked
```

得到兩個重複群組：三份 Rust LF（各5bytes）、兩份空檔；不同的 rust LF 不加入前一群組。這是固定教材 root 的階段示範，尚未交付完整參數與0／1／2狀態。

核心保存原生 PathBuf。一般檔案失敗留下 FileIssue，確切失敗者移除後才重建其餘候選；錯誤不當成不同內容。讀取前後的路徑與 handle 觀察屬盡力而為，沒有一致快照保證。
