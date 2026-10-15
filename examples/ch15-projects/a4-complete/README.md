# A4 完成快照

這份答案已接成可用CLI：一個原生root、可選--json、預設穩定文字，兩種格式取自同一份Report。完整報告成功交出回0，部分報告成功交出回1，致命root/指標/JSON/輸出錯誤回2。write_all後明確flush，任何一步失敗都不回0／1。

從教材repo根目錄：

```bash
cargo test -p duplicate-a4 --locked
cargo run --quiet -p duplicate-a4 --locked -- examples/ch15-projects/materials/duplicate-demo
cargo run --quiet -p duplicate-a4 --locked -- examples/ch15-projects/materials/duplicate-demo --json
```

root依真正CWD解讀，--manifest-path不會改工作目錄。六份固定材料產生10bytes／兩群與狀態0；指定自己的目錄即可使用。工具只報告，不刪除；no-links、原生path identity、非Unicode問題與changed-best-effort政策沿用前文。

Unix應有29項既有加6app與6CLI，共41項；其他平台不編入七項Unixfixtures，作者沒有在非Unix執行。app新tests與工作版checks逐bytes相同；integration除crate／binary名稱外同樣，使用本package的CARGO_BIN_EXE_duplicate-a4。

一般項目問題放在報告的issues裡，保持整份JSON可解析；fatal診斷送stderr。輸出失敗時stdout可能已有部分bytes，狀態2不宣稱那些bytes是成功交付的報告。fatal診斷若也不可寫，仍回2，不panic。

18.6再整理完整參考版的API邊界、文件與整份驗收；此階段已能使用，但不把重複stagecopies計入最終專案行數。
