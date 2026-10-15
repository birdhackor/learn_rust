# A1 完成快照

這是第 18.2 節的收集階段參考版，只有 library 與驗收；不是完整重複檔案工具。

在教材 repo 根目錄可執行：

```bash
cargo test -p duplicate-a1 --locked
```

教材的 demo 入口固定如下；相對路徑是從目前工作目錄解析，不是從 source 檔解析：

```bash
cd examples/ch15-projects/a1-complete
cargo run --example inventory --locked
```

它只顯示六個待判定檔案、byte 長度與項目問題。這個示範 main 沒有實作完整工具的 0／1／2 結束狀態或可配置 root，這些交給 A4。

`src/collection.rs` 末尾的八項驗收來自工作版 `checks/collection-tests.rs`。Unix 的三項 link／非 Unicode 材料只在對應平台編入。
