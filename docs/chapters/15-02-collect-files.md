# 15.2 A1：自己收集檔案與項目問題

上一節固定了重複檔案搜尋器的範圍，也準備了六份真實材料。這一步先回答：**哪些一般檔案可以交給後面的內容判定？哪些項目沒能完成收集？** 完成 A1 時，六份材料都要成為檔案記錄，包含 different 與兩個空檔；還不把其中任何幾份合併成重複群組。

這次組合 9.3 已教的 metadata、walkdir、原生路徑與錯誤政策。先自己完成三個函式，再看本階段快照；完整工具留到後面的里程碑。核心放在 library，方便直接驗收，不先把 CLI、內容比較與報告一起寫進 main。

## 固定這一步的輸入與答案

`scan(&Path)` 接收一個 root，交回 `Result<ScanReport, RootError>`。ScanReport 分開持有一般檔案 files 與項目問題 issues。RootError 表示工作根本不能開始，不是一份成功的空清單。

沿用 9.3 的政策，請先把以下預期寫在自己的筆記：

| 材料或情況 | 本階段的答案 |
| --- | --- |
| 15.1 的六份材料 | files 有六項；四項長度為 5 bytes，兩項為 0；issues 為空 |
| 有效的空目錄 | 成功，files 與 issues 都為空 |
| root 不存在／是一般檔案／是 link | RootError，不能開始收集 |
| 子目錄與點開頭的隱藏目錄 | 繼續往內走，不把目錄本身存成 FileRecord |
| 樹內 link 與其他特殊項目 | 依政策略過，不追蹤；略過不算失敗 |
| 走訪／一般檔案觀察失敗 | 保存 ScanIssue，繼續其他項目 |
| 一般檔案的路徑不能表示為 Unicode | 保留原生位置與 NonUnicode 問題，不加入 files |

files 最後按原生路徑排列，沿用 9.3 在固定平台上使用 Path 的 Ord；不是用 display 文字作身分，也沒有跨作業系統相同排序的承諾。你現在保存的長度與修改時間只是觀察資料，尚未證明內容相同或不會變動。

## 把工作版放進自己的資料夾

繼續使用 15.1 在 repo 外建立的 duplicate-finder。這一階段只建立 library：用編輯器移除 Cargo new 產生的 `src/main.rs`，再保存以下三份起始檔。原來的資料夾名稱可以不變；Cargo 以 manifest 的 package name 管理此階段，兩者不是一定要相同。

起始檔位於教材的 `examples/ch15-projects/a1-starter`。請從 [A1 工作版目錄](https://github.com/birdhackor/learn_rust/tree/main/examples/ch15-projects/a1-starter) 保存上述三份完整檔案到自己的練習資料夾；下方節錄用來定位介面與待完成處。collection.rs 的 imports、模型及 Display 也包含在完整起始檔中。先保存自己的副本，後面的答案不會替你改掉它。

manifest 採獨立的 package 欄位，不依賴教材 workspace 的繼承設定。沿用 7.1 的版本要求：`"2.5"` 與 `"3.27"` 表示可接受的同 major 版本範圍，解析到的確切版本由 Cargo.lock 保存。walkdir 是 library 的走訪依賴，tempfile 放在 dev-dependencies，只供 fixture 驗收使用：

```toml
--8<-- "examples/ch15-projects/a1-starter/Cargo.toml"
```

`src/lib.rs` 宣告這次的 collection module，沿用 1.9 的檔案與 module 對應：

```rust
--8<-- "examples/ch15-projects/a1-starter/src/lib.rs"
```

`src/collection.rs` 先保存完整的起始檔。它開頭使用與 9.3 相同的 Snapshot、FileRecord、ScanCause、ScanIssue、ScanReport、RootError：成功記錄擁有 PathBuf；ScanIssue.path 是 Option，因為走訪失敗不一定提供位置；action 是固定 literal 的 `&'static str`，沿用 8.4 的 lifetime。模型與 Display 已提供，這次要自己寫的是如何取得與分流資料。

這三個待做函式各有一項責任：

- `validate_root`：觀察 root 本身，拒絕不存在、link 與非目錄；驗證成功後才開始走訪。
- `observe`：重新觀察一般檔案，取得長度與修改時間或保留原因；collect_entries 決定存記錄或問題。
- `collect_entries`：接收逐筆 Ok／Err，處理政策，分別累積並排序；ScanReport 交給後面的內容判定。

起始 body 只標記未完成位置，沒有隱藏的完整演算法。名稱開頭的 `_root`／`_path`／`_source` 暫時沒有被使用，沿用先前以底線前綴表明不用的參數；開始實作時可改回不帶底線的名字。

```rust
--8<-- "examples/ch15-projects/a1-starter/src/collection.rs:work"
```

**validate_root 固定回傳 NotDirectory，並沒有真的看 root；observe 固定拒絕，collect_entries 也沒有走來源。** 不要把它們的暫時結果解讀成材料本身的判定。已提供的 scan 只負責接上三份工作：

```rust
--8<-- "examples/ch15-projects/a1-starter/src/collection.rs:wire"
```

兩個 false 沿用 9.3：follow_links 管樹內遇到的 link，follow_root_links 管 root 本身。驗證與走訪之間仍可能有外部變動；這些設定不會把一般檔案整理工具變成抵抗同時替換路徑的保證。

先從自己的練習資料夾解析依賴、檢查起始檔能編譯：

```bash
cargo +1.99.0 check
```

這還沒有加入驗收，check 成功只證明介面能編譯。

## 先加入驗收，看清未完成的地方

工作版另有 `checks/collection-tests.rs`。**checks 不是 Cargo 自動辨認的 tests 目錄**；這份檔案目前只是要加入的材料。用編輯器把其中整個 tests module 加到自己的 `src/collection.rs` 末尾，保留既有函式；module 前的條件屬性也要一起保留。它沿用 1.10 的同檔 unit tests；`crate::collection` 從本 library 的 crate 根開始，不需要另外猜外部 crate 的名稱。

第一個要理解的材料測試，重建上一節的六份檔案。TempDir owner 保留到掃描完成才清理；create_dir、write、join 都已在 7.8／9.3 用過。它先確認六個原生位置與長度，再比較排序後的路徑清單：

```rust
--8<-- "examples/ch15-projects/a1-starter/checks/collection-tests.rs:material-test"
```

for 借用 report.files 以檢查長度，之後 into_iter 才接手記錄，map 移出每個 path，collect 成 `Vec<PathBuf>`。這是 6.5–6.6 的 ownership 與收集分工；這裡不用顯示文字重新建立路徑。

加入整份驗收後，從自己的工作資料夾執行：

```bash
cargo +1.99.0 test --locked
```

起始版應出現**行為失敗**。例如 six_material_files 這項測試會在期待合法 root 成功時得到固定 NotDirectory；其他測試還會發現沒有保留 walker 錯誤。這和找不到 module、缺少 import、lock 尚未更新的編譯或設定失敗不同。先修好檔案與設定問題，才用 assertion 回饋完成函式。

observe 拒絕目錄的測試此時可能已通過，因為 stub 暫時拒絕所有位置；這一項通過沒有證明它能接受一般檔案。也不要刪掉失敗測試或改掉預期值，讓起始版看起來完成。

## 自己完成三項工作

建議照 root → 單檔觀察 → 逐筆收集的順序做；每步改一個函式，重跑同一組驗收，觀察哪些失敗原因改變。三份工作都用 9.3 已教的 std::fs；開始實作時，在 collection.rs 的 module import 區加入 `use std::fs;`。

1. **validate_root：** 使用不追蹤最後 link 的查詢。查詢失敗保留原 I/O error；root 是 link 與不是目錄要分別回傳模型中的原因。有效目錄才回 Ok。
2. **observe：** 再查當下位置，先確認一般檔案。取得長度與修改時間都屬這次觀察；modified 若失敗也交回原因，不能猜一個時間或直接略掉檔案。
3. **collect_entries：** 建立 files／issues，消耗每個 Result。錯誤記錄後繼續；成功 entry 再判類別、Unicode 與 observe，最後保存 owned 原生路徑。完成整個來源後按 path 排 files。

注意第三步的 Result 有兩個層次：**scan 的 Err 是 root 不能開始，來源 item 的 Err 是已開始工作裡的一項問題。** 在 collect_entries 遇到 Err 時直接提前交回空清單，會漏掉後面的成功；先丟掉 Err 再累積 files，則會讓報告失去不完整的證據。

模型、需求與所需 API 已在主文給定。先自己寫；卡住時，再依需要打開下一層提示。

??? hint "提示 1：先追資料的去向"

    先畫兩欄：可供內容判定的檔案、沒能完成的項目。遇到問題，後面仍需要知道位置、動作與原因，所以不是只計數或 println。root 失敗位於這份清單開始前，交回 RootError；來源中的問題進 issues。

    空檔有合法的觀察結果，應進 files。link 是我們約定不處理的對象，通常不用新增問題。非 Unicode 一般檔案則超出最小報告能力，應留下問題，不能悄悄消失。

??? hint "提示 2：回收已教的能力"

    symlink_metadata 觀察最後項目本身；file_type 的 is_symlink 與 metadata 的 is_dir 決定 root。observe 使用 metadata 的 is_file、len、modified。取得 modified 的 Result 仍需處理。

    DirEntry 的 file_type 不額外查詢；path 借出 &Path，into_path 交出 owned PathBuf。先用 borrowed path 做判斷，確定存到哪一欄後，再 move。

    error.path 可能沒有位置；Option::map 配合 to_path_buf 把可能存在的 borrowed 位置轉成 owned 副本。Unicode 判斷用 to_str 的 Option；sort_by 的 callback 借用兩份記錄，交回 path.cmp 的順序。

??? hint "提示 3：只拆開一個分支"

    先完成來源 item 的 Err 分支：從 error 借出可能的 path、建立 owned 副本，組成 ScanIssue，action 用「走訪目錄」，cause 保存原 walkdir::Error，push 後 continue。這樣後面的 Ok 仍有機會處理。

    Ok 分支還有三個不同決策，留給你完成：這是一般檔案嗎？路徑可供本題報告處理嗎？重新 observe 成功嗎？把每次借用與 move 的先後寫在紙上，比一次塞進長 iterator chain 更容易核對。

## 核對完整 A1 的行為

驗收不是只看總數。完整材料另有下列能分辨錯誤做法的檢查；若某項失敗，先用它指出的政策找對責任函式：

| 檢查 | 為什麼需要 |
| --- | --- |
| 六份材料、隱藏子目錄、空檔、原生路徑排序 | 避免遺漏資料、提前刪掉長度為零者，或把顯示當身分 |
| 有效空目錄／不存在 root／root 一般檔案 | 分辨沒有檔案與不能開始 |
| observe 收到目錄 | 走訪時的類別不會固定未來；重新觀察要守住一般檔案要求 |
| 受控 Ok／Err／Ok | 中間問題有保存，最後一份成功仍被收集 |
| Unix：root link、內部 file／directory links | 根拒絕與樹內不追蹤是不同位置的政策 |
| Unix：非 Unicode 檔名與一個合法檔名 | 保留原生錯誤位置，其他項目仍成功 |

受控 Ok／Err／Ok 沿用 9.3：分別從一般檔案、不存在位置、另一份一般檔案取得 walker item，再交給 collect_entries。這是在測收集元件，不是讓正式 scan 接受 file root；不依賴目前帳號剛好沒有讀取權限。

Unix fixtures 只在 `#[cfg(unix)]` 條件下編入，使用 9.3 已教的 symlink 與 OsStringExt::from_vec。其他平台仍驗收前五項通用測試；未執行的 Unix 項目不能算成該平台已驗過。metadata 的 modified 無法提供時如何保存，仍須對照 observe 的分支，這組 fixture 沒有控制這種平台失敗。

在 Unix 上，八項測試都應通過；這支持 A1 列出的材料與政策，不證明同時變動時有一致快照。完成後也跑 1.10 的 fmt／Clippy。先保留自己的版本，再按需要查看答案。

## 本階段完成快照與真實材料

參考版是 [A1 完成快照目錄](https://github.com/birdhackor/learn_rust/tree/main/examples/ch15-projects/a1-complete)，package name 為 `duplicate-a1`。`src/collection.rs` 保存本階段完整實作與同一份驗收，沒有 A2 的內容演算法。需要對照時，先找目前仍失敗的那個函式，而不是覆蓋自己的整份工作版。

在教材 repo 根目錄，這份 snapshot 可以真正執行驗收：

```bash
cargo test -p duplicate-a1 --locked
```

另外準備了 inventory example，沿用 9.3 的資料顯示：借用選定 root、收集後列出長度與路徑，項目問題交到 stderr。這裡選用 **`eprintln!`**：它接受和 println! 相同的格式字串與參數，將一行診斷寫到 stderr；寫入失敗會 panic。這份固定材料的階段示範仍沒有完整 CLI 的錯誤與狀態政策。它的相對 root 是 `../materials/duplicate-demo`，因此**操作入口固定為這個 package 的目錄**；不是相對於原始碼位置。在教材 repo 根目錄開始：

```bash
cd examples/ch15-projects/a1-complete
cargo run --example inventory --locked
```

第一行資料應為：

```text
A1：6 個待判定檔案，0 個項目問題
```

接著是六個路徑，包含 .saved／sub 內的檔案；空檔標為 0 bytes，其他標為 5 bytes。此時 different 仍在 files 裡，因為它完全符合收集政策。這個固定入口的 demo 沒有實作最終工具的參數與 0／1／2；完整 CLI 在 A4 接上。

??? note "只在需要時對照三個函式"

    以下內容來自可執行的 A1 snapshot；模型與函式介面和工作版相同。

    ```rust
    --8<-- "examples/ch15-projects/a1-complete/src/collection.rs:root"
    ```

    ```rust
    --8<-- "examples/ch15-projects/a1-complete/src/collection.rs:observe"
    ```

    ```rust
    --8<-- "examples/ch15-projects/a1-complete/src/collection.rs:scan"
    ```

現在可以把成功的檔案記錄交給下一階段。保留 issues，因為後面的判定即使找到重複群組，也不能抹掉前面已有的未完成項目。

延伸閱讀：[English Rust Cookbook：Directory Traversal](https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html)、[中文 Rusty Book：目錄訪問](https://rusty.course.rs/files/dir.html)、[walkdir 2.5：WalkDir](https://docs.rs/walkdir/2.5.0/walkdir/struct.WalkDir.html)、[DirEntry](https://docs.rs/walkdir/2.5.0/walkdir/struct.DirEntry.html)。前兩者示範用 library 管理走訪，本題自行保留失敗、root 與 Unicode 政策，不採丟棄 Err 的簡短寫法。
