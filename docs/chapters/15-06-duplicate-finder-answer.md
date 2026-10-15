# 15.6 A 完整參考版：把練習收成可用工具

A1～A4 已分別完成收集、真正內容分組、報告與 CLI。這一節把它們保存成一份可用的 [duplicate-finder 完整參考版](https://github.com/birdhackor/learn_rust/tree/main/examples/ch15-projects/duplicate-finder)。需要答案時從這裡進入；想自己練習時，前面四份工作版、分層提示與驗收仍保留。

你不必一次重寫所有 stage，也不必把每階段重複的支援函式和測試再寫一次。完整版本的核心 995 行，另有 662 行驗收與 fixture，合計 1,657 行（含空行、註解與文件測試）。核心約千行，驗收另供參考，不是一次新增一千多行的練習。這些數字只計這一個 package，包括 tests 與文件測試，不加 lock、建置產物或前面四階段副本。

## 先核對這仍是原本那個題目

它接受一個 root，不跟 symbolic links，先以大小與 BLAKE3 篩候選，再以 bytes 確認真正相同。摘要桶可以有多個真內容群；遇到失敗，保存確切原生位置，移除失敗者後重新判定剩餘候選，不能漏掉仍然相同的 B／B。

原生 PathBuf 是核心的身分與排序材料。不能轉成 Unicode 的項目留下問題，成功部分仍可報告；Debug 位置只供顯示。讀前／讀後的 metadata 觀察是盡力判定變動，沒有一致快照保證。按不同路徑算的重複 bytes 也不是可回收的實體空間，hard links 仍按不同路徑計數。

文字與 JSON 表達同一 Report：穩定順序、群組、指標、兩種問題與 complete。write_all 與 flush 都成功後，完整回 0、部分回 1；無法開始、建出正確數字、編碼或完成輸出則回 2。工具只報告；沒有刪除、跟連結、平行掃描或新的輸出格式。

## 自己做最後一小步：收好公有入口

14.4 已教 private module 與 pub use。現在先想 caller 需要什麼：CLI 需要 Settings 與 run；另一份程式可能只想借一個 Path 取得報告，並從 RunError 分類失敗。它不需要知道摘要桶、reader fixture 或失敗重建 callback 放在哪個檔案。

完整版本保留私有 modules，在 library root 重導出已完成的型別與函式。RunError 的 public variants 會保存 RootError／ReportError，後者又保存 MetricError，這些公有 payload 型別也有可用的 root 名稱。Report／ReportGroup／Diagnostic 是 owned 結果資料；公有欄位方便讀取、比較和 JSON 往返，沒有宣稱外部任意改欄位仍是一份正確掃描。

先把自己的入口照這份責任整理，再比較答案。pub use 重導出的仍是同一個型別，沒有把資料重新包一層，也沒有讓 private module 本身變公有：

```rust
--8<-- "examples/ch15-projects/duplicate-finder/src/lib.rs"
```

這是另一個完整參考 package 的介面。沒有說直接刪除 A4 既有的公有 module 路徑也會保留舊 caller 的相容性；14.4 已分開來源和行為契約，四個 stage 的原 artifact 仍保持原樣。

### 讓不使用 clap 的 caller 也能借 root

scan_report 接收 `&Path`，沿用 A4 的 scan→analyze→build_report，回傳 owned Report。root 失敗或指標不能成立時是 Err；項目問題仍是 Ok 的部分報告，caller 讀 complete 與 issues 決定下一步。它不選輸出格式，也不寫 stdout。

自己先把這三步從 run 抽成薄函式，再讓 run 呼叫它。下面的文件測試沿用 7.2／14.4 的 `///`，以 7.8 的 TempDir 建立真實空目錄；它是正常完整的零結果，不靠讀者機器上某個固定路徑：

```rust
--8<-- "examples/ch15-projects/duplicate-finder/src/app.rs:scan-api"
```

## 在報告邊界借讀標準原因

8.6 的 std::error::Error 提供 Debug／Display 與 source。Result 本來就能保存具體錯誤，所以不用把每個內部 enum 都加同一套 impl；這次只讓公有 RunError 支援共同報告介面，原 typed variants 仍保留。

先核對本次選用的能力：io::Error、TryFromIntError 和 serde_json::Error 都提供 Error；它們保存的資料符合 source 所需的型別條件。RunError 借出已有標準原因，source 的外層 reference 仍借這次 self；`'static` 沒有把它變成永久 reference。

這裡 source 跳過沒有 Error 介面的分類 wrapper，直接借到標準原因。Root 的 I/O、指標的 Count、JSON 與 Output 有下一層；根不是目錄、根是 link、數字 Overflow 等自己已說明原因的結果則回 None。

Display 與 source 分工：有標準原因時，這層只說正在失敗的工作，下層原訊息由 source 顯示；沒有下一層時，Display 保留完整 leaf 意義。enum 沿 14.4 的 non_exhaustive，外部分類仍保留其他分支。先在自己的 RunError 寫這份借用關係，再對照：

```rust
--8<-- "examples/ch15-projects/duplicate-finder/src/app.rs:error-boundary"
```

binary 的 explain 借 `&dyn Error`，沿 8.6 已教的有限原因鏈取得 owned String；沒有改 Result 的分類，也沒有接手原錯誤。這份報告器只用共同的 Display／source，從每一層往下借，遇到 None 結束。完整工具自己的標準原因沒有循環回 RunError：

```rust
--8<-- "examples/ch15-projects/duplicate-finder/src/main.rs:explain"
```

原本 fatal 的 stderr／狀態 2 與 write_output 的明確 flush 沒有改。參數資訊仍用 clap 已教的 Display／出口角色；它不是 scan_report 的檔案錯誤。

### 私有 callback 的 panic 是什麼意思

A2 的重建要求 callback 報出本次候選的真正 offender，移除後才有有限進展。違約時的 panic 是程式作者把 callback 接錯的防守；完整公有入口不讓使用者替換這份 callback。一般檔案失敗依舊保存 ScanIssue／FileIssue，繼續其他項目，最後交 partial1。

不要把這個私有 programmer 契約改成「檔案讀不到就 panic」，也不要把真的讀取失敗回 false，假裝它只是不同內容。private API 的安排限制 caller 可以做什麼；正確的錯誤政策仍由實作與驗收支持。

## 用一份完成版與一份驗收查答案

| 前面的成果 | 完整版本中的位置 |
| --- | --- |
| A1：走訪、原生位置、觀察與收集問題 | collection.rs |
| A2：摘要、短讀取、真正比較、失敗者重建 | stream.rs／files.rs／content.rs |
| A3：checked 指標、原生排序、兩種報告 | metrics.rs／report.rs |
| A4：設定、輸出完成、0／1／2 | app.rs／main.rs |
| 最後入口與公開契約 | lib.rs、scan_report 文件測試、tests/public_api.rs |

最終驗收把重疊材料合併，保留能揭示不同關係的檢查。`src/tests.rs` 是 crate 內測試，可以檢查私有 grouping helper；`tests/cli.rs` 真正啟動 binary；public_api 和文件測試只走公有 root 入口。callback 測試材料可以使用受控摘要或 reader，但正常 CLI 始終使用真的 BLAKE3 與檔案比較。

最終 Source helper 只借用測試中仍有效的 bytes，省去每次建立來源時的資料複製；4.5 的 lifetime 省略把回傳來源接到這份輸入。width 控制每次正數短讀，interrupted 開關先回一次 Interrupted；fail_at 則控制接收資料後的 PermissionDenied。所有驗收的 width 都是正數。它合併 A2 兩個可見 reader 的責任，沒有另開網路、猜檔案權限或用 sleep 等變動。Output helper 同樣保留 A4 的寫入／flush 分別失敗；人工 Report helper 只測表示與指標，不能當成讀過磁碟的證據。

| 合同 | 保留的可觀察判準 |
| --- | --- |
| 不跟 links、保留 hidden／empty／Unicode 名 | 真 child 的已知材料與 link fixtures；空 root 是完整0 |
| size→digest→bytes | singleton 不做 hash；強制一個 digest 桶仍得多真群；重複原生位置不算兩個檔案 |
| 讀取錯誤與部分成果 | Interrupted 重試後真的失敗，保存左／右來源；移除 A 後 B／B 仍成立 |
| 相同內容的判準 | 3／7 正數短讀跨 buffer；非 UTF-8、空 bytes、不同尾 byte、缺尾資料分開判 |
| 變動只是盡力判定 | 改長度的讀前與讀後觀察，保存 native offender；仍有其他成功配對 |
| 固定表示與正確數字 | 原生順序、所有診斷 ties、控制字元與 JSON 往返、乘法與加法溢位 |
| 邏輯 bytes | 真 hard links 仍有兩個不同路徑，指標3，不宣稱物理可回收量 |
| 出口與錯誤介面 | text／JSON、0／1／2、write／flush 失敗、typed Root 原因與借用 source |

這些不是要求你現在一次重寫全部 tests。沿 A1～A4 做過一輪後，最後用它們確認整合沒有改壞原題目；某項失敗時回到對應責任，再查那一段完整答案。

## 建置、使用與查閱

從教材 repo 根目錄，沿第 1 章已完成的環境執行：

```bash
cargo build -p duplicate-finder --release --locked
cargo test -p duplicate-finder --locked
cargo run --quiet -p duplicate-finder --locked -- examples/ch15-projects/materials/duplicate-demo
cargo run --quiet -p duplicate-finder --locked -- examples/ch15-projects/materials/duplicate-demo --json
```

六份固定材料的答案仍是 complete=true、兩群、10 bytes、空問題清單與狀態 0。換成自己的 root 即可；root 依實際 CWD 解讀，--manifest-path 不會替你改工作目錄。help／version 沿用 `--help`／`--version`。

[完整 README](https://github.com/birdhackor/learn_rust/blob/main/examples/ch15-projects/duplicate-finder/README.md) 保存同一套操作、獨立資料夾的 toolchain／lock 起點、JSON 欄位與限制。manifest 仍是 literal，可以連同 source 保存到自己的練習資料夾；它不是只能依賴本 repo 隱藏設定才能用的答案。

文件也能作為入口查詢；沿 7.2／14.4 的 Cargo doc，只產生這份 package 的文件：

```bash
cargo doc -p duplicate-finder --no-deps --locked
```

Unix 最終應有 8 項核心、7 項 CLI、1 項外部 API 與 1 項文件測試，共 17 項。部分測試檢查多組材料，數量沒有和前面 stage 一一累加；其他平台不編入三項 Unix CLI fixtures。

完成這一題後，你已把基本語法、ownership、trait、iterator、I/O、Cargo、測試與錯誤介面接成一份可以使用的程式。要參考答案時，從完整 package 的 README 和公有入口開始，再按上表找到實作；要繼續練習時，保留自己的版本，使用同一份判準核對結果。

延伸閱讀：[English Book：Public API](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api)、[簡中對應](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)、[std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)、[serde_json::Error](https://docs.rs/serde_json/latest/serde_json/struct.Error.html)。本題使用公有入口與共同原因借用的安排，完整工具與前面各段實作都保留自己的已教契約。
