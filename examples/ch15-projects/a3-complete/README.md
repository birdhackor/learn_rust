# A3 完成快照

這份 library 在 A2 的真正內容分組與項目問題之上，建立同一份 owned Report，提供穩定文字與 JSON。保留 native PathBuf 到排序完成，Unicode group 與 Debug 診斷只是輸出表示。兩種 issues 都保留；任何一項問題使 complete=false。

從教材 repo 根目錄：

```bash
cargo test -p duplicate-a3 --locked
```

Unix 有 29 項測試；其他平台不編入五項 Unix fixtures。新增 A3 module 與工作版 checks/report-tests.rs 相同。非 Unix 數量由條件選取預期，作者沒有在非 Unix 執行。

固定六檔示範入口：

```bash
cd examples/ch15-projects/a3-complete
cargo run --example a3_text --locked
cargo run --example a3_json --locked
```

兩份報告都表示 complete=true、重複內容10bytes、兩個群組、空問題清單。示範 write_all 後明確 flush；expect 只依固定教材材料，完整 CLI 的錯誤與 exit 政策仍待 A4。

指標按不同路徑算，hard-link 路徑也計入，沒有宣稱可回收同量實體空間。乘法或合計溢位回 typed ReportError，不輸出錯誤數字。工具沒有刪除功能。
