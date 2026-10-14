# 9.3 走訪檔案，保留路徑與失敗

7.5 已能把一個原生路徑交給讀檔工作。現在把輸入改為一個目錄：要列出它與子目錄中的一般檔案，包括隱藏目錄內的檔案，保存原生路徑、byte 長度及修改時間，供後面的內容工作使用。

這次先不讀內容，也不判重複。成功清單旁邊還要保存失敗項目；否則遇到不可處理的檔案時，「沒有列出」可能被誤認為「不存在」。**根設定不成立則整個工作無法開始；個別項目失敗則保留原因，繼續其他項目。**

本節 package 是 `ch09-files`，位於 `examples/ch09-files`。Cargo 命令都從 repo 根目錄執行。先訂出這次的檔案政策，再選 API。

## symbolic link 是另一個位置的指示

**symbolic link（符號連結）** 是檔案系統中的一種項目，保存指向另一個位置的資訊。它可以指向檔案或目錄，也可能指向不存在的位置。追蹤 link 表示改去處理指向的目標；若目錄 link 又指回上層，追蹤還可能遇到循環。

本例要掃描使用者選定的目錄樹，故採用以下規則：

| 位置 | 本例處理方式 | 對工作範圍的作用 |
| --- | --- | --- |
| root 是 symbolic link | 根設定錯誤，停止 | 不把使用者給的 link 默默換成另一份目錄樹 |
| root 是一般檔案／不存在 | 根設定錯誤，停止 | 本次入口要求目錄 |
| 目錄裡的一般檔案 | 保存記錄 | 後續可以開啟與讀取 |
| 子目錄，包括隱藏目錄 | 繼續往內走訪 | 不以名稱開頭的點自行排除 |
| 目錄裡的 link／其他特殊項目 | 略過，不追蹤 | 政策允許的略過不算項目失敗 |

「不是目錄」不能當成「一般檔案」；link 與其他特殊項目也可能不是目錄。因此後面會明確選 `is_file()`。

## metadata 是觀察到的檔案資料

**metadata** 是檔案系統回報的資料，例如項目類別、長度與修改時間。它不是檔案內容，也不是替檔案上鎖。

標準函式庫 `std::fs` 提供兩種查詢：`metadata(path)` 會追蹤最後的 symbolic link，`symlink_metadata(path)` 則查最後這個項目本身。兩者都接受原生路徑，回傳 `io::Result<Metadata>`，可能因不存在或權限等原因失敗。要拒絕 root link，必須用後者。

本例選用以下 methods，全部 shared 借用 metadata：

| method | 結果 | 本例用途 |
| --- | --- | --- |
| `file_type()` | `FileType`，描述項目類別 | 再以 `is_symlink() -> bool` 檢查 link 本身 |
| `is_dir()` | `bool` | root 必須是目錄 |
| `is_file()` | `bool` | 後續只接受一般檔案 |
| `len()` | `u64`，byte 數 | 檔案長度，不是字元數或 buffer 索引 |
| `modified()` | `io::Result<SystemTime>` | 保存修改時間；有些平台或檔案系統無法提供，失敗也要保留 |

`u64` 是無號 64-bit 整數，從零到它能表示的最大值；沿用 1.4 的有限範圍概念。`usize` 用於目前程式的索引與清單長度，metadata 選擇的 `u64` 則用於檔案 byte 長度；兩者不會自動互換。

`std::time::SystemTime` 表示一個由系統回報的時間點。本節只保存它，之後用相等比較觀察修改時間是否不同，不換算經過秒數。這個型別有 `Debug`、`Clone`、`Copy`、`PartialEq`，所以能顯示、保存副本及比較。同樣 byte 長度不代表內容相同，時間相同也不是內容不變的證明。

### 先把成功資料與失敗資料分開

library 的 `scan` module 定義下列資料。`Snapshot` 保存這次觀察的 `len`／`modified`，名字不承諾一致的檔案快照。`FileRecord` 再持有原生 `PathBuf`。7.5 的 owner／reference 關係仍然成立：這份記錄擁有路徑文字，沒有接管或固定磁碟上的檔案。

`ScanCause` 保存具體原因：I/O 錯誤、walker 錯誤、不能轉成 Unicode、或觀察時已不是一般檔案。walker 選用的 `walkdir` crate 以具體 `walkdir::Error` 保存走訪失敗，這個型別已有 `Debug`／`Display`，可以檢查和顯示原始原因。`ScanIssue.path` 用 `Option<PathBuf>`，因為 walker 失敗不保證能提供路徑；`action` 是程式中固定的 `&'static str`，沿用 8.4 的 literal lifetime。Display 沿用 5.3，保留兩種原始錯誤的顯示方式。

`RootError` 描述整個工作開始前的失敗。它和 `ScanIssue` 不能混成一份成功空清單，否則「root 不能用」會被誤當成「root 裡沒有檔案」。

```rust
--8<-- "examples/ch09-files/src/scan.rs:records"
```

把觀察一般檔案的工作先寫成 `observe`。它重新查詢不追 link 的 metadata；即使上一個步驟看過一般檔案，目前位置也可能已改變。查詢、類別與修改時間取得都是可能失敗的步驟：

```rust
--8<-- "examples/ch09-files/src/scan.rs:observe"
```

## 用 walkdir 逐筆走訪，Item 也可能是錯誤

自己管理每層目錄的待辦也能完成走訪。這裡選 **`walkdir`** crate，把逐層進入目錄與保留走訪進度交給它；我們專注於項目政策和輸出資料。依 7.1 在 manifest 宣告依賴，測試則回收 7.8 的 `tempfile`：

```toml
--8<-- "examples/ch09-files/Cargo.toml"
```

`walkdir::WalkDir::new(root)` 接受原生路徑，交回一份走訪設定；它有 `IntoIterator`，所以 6.5 的 `for` 可以接手這份工作。逐筆的 **Item 是 `Result<walkdir::DirEntry, walkdir::Error>`**。`walkdir::Error` 已有 Debug／Display，供前面 `ScanCause` 保存和顯示。遇到錯誤會交出 Err，仍可繼續下一項；若某個目錄打不開，就不能取得那個目錄內的內容。

走訪順序沒有供報表使用的固定保證。本例收集完成後，才用 6.7 的 `sort_by` 指定順序，不從「這次剛好先看到哪個檔案」推定下次相同。

### 兩個 link 開關看不同位置

`follow_links(false)` 接手設定、回傳更新後的設定，控制**裡面遇到的** link；預設就是 false。另一個 `follow_root_links(false)` 控制**root 自己**的 link，其預設是 true。因此只寫前者不能完成我們的 root 政策。

先用標準 `symlink_metadata` 驗證 root 是一般目錄，遇到 link 直接交回設定錯誤：

```rust
--8<-- "examples/ch09-files/src/scan.rs:root"
```

walker 再明寫兩個 false。驗證與走訪之間外部程式仍可能改動檔案系統；這個設定避免 walker 接著主動追 root link，沒有把前面的檢查升級成抵抗惡意同時替換的安全保證。這節適用於穩定目錄上的一般檔案整理。

### DirEntry 的借用與 ownership

| API | 結果與 ownership | 本例用法 |
| --- | --- | --- |
| `entry.file_type()` | 取得項目類別，沒有額外 metadata 查詢 | 用 `is_file()` 排除目錄／link／特殊項目 |
| `entry.path()` | `&Path`，借用 entry 保存的原生路徑 | 判 Unicode、交給 observe |
| `entry.into_path()` | 接手 entry，交回 owned `PathBuf` | 把位置保存到結果，之後不用原 entry |
| `error.path()` | `Option<&Path>`，借用錯誤中的位置 | 有位置才保存 owned 副本；None 如實留下 |

`Path::to_path_buf()` 已在 7.8 教過，是從 borrowed Path 建立 owned 路徑副本。DirEntry 的類別資訊是走訪時取得的，不會讓磁碟永遠保持同一類別；因此通過初步篩選後，仍由 `observe` 檢查當下位置。

這裡另外用 **`Option::is_none()`**：shared 借用 Option、回傳 bool，只有 None 才是 true。它檢查的是能否表示路徑，不會自動改造資料。

為了保存錯誤裡可能有的路徑，再選 **`Option::map(callback)`**：接手 Option，Some 時將內容交給 `FnOnce` callback，再把回傳值包成 Some；None 時不執行 callback，仍交回 None。因此把 `Option<&Path>` 轉成 `Option<PathBuf>` 可以只寫 `map(|path: &Path| path.to_path_buf())`。這與 6.6 逐筆 iterator map 的數量不同，只處理零或一份資料；沒有丟棄原始 walkdir 錯誤。

## 原生路徑保留身分，顯示只供人讀

7.5 已教 `Path::display()` 可能無法忠實表示非 Unicode 路徑。兩份不同原生資料可能顯示成相同文字，所以不能把 `format!("{}", path.display())` 當成核心身分或再拿去開檔。

本例核心一直保存 `PathBuf`；報告的最小政策則只接受 `to_str()` 成功的檔案路徑。失敗者保留原生位置、動作與 `NonUnicode` 原因，跳過該檔案，其他項目繼續。錯誤顯示以 `Debug` 檢查原生資料，不悄悄改成另一個檔名。

`Path`／`PathBuf` 也有 4.3 已教的 `Eq`、`Hash` 和 6.7 的 `Ord`。它們依原生路徑組成比較、分組與排序，沒有查磁碟證明是同一個實體檔案。`PathBuf` 以 `Borrow<Path>` 提供對應 borrowed key，因此 `HashMap<PathBuf, u64>` 可以用 `&Path` 查詢。這和 4.3 用 `&str` 查 `String` 鍵是同一份身分一致性的契約。

現在才把流程接起來。`scan` 驗證 root，再把 walker 交給 `collect_entries`。後者沿用 6.5 的泛型 `IntoIterator<Item = ...>`，只要求來源交出相同形狀的成功／失敗項目，回傳 `ScanReport`；它不再驗證 root，因為輸入已是一份逐筆工作。這份分工也讓測試能提供可控制的錯誤順序。

每個 Err 保存後 `continue`，沒有 `filter_map(Result::ok)` 丟棄失敗；最後按原生 Path 的 `cmp` 排一般檔案。這是固定平台上的固定排列，不承諾所有作業系統的路徑順序一致：

```rust
--8<-- "examples/ch09-files/src/scan.rs:scan"
```

main 顯示成功資料與項目錯誤，最後以 7.4 已教的 slice `is_empty()` 判斷有沒有項目失敗。這裡 `for` 接手 `report.files` 欄位，但仍可借用互不重疊的 `report.issues`。這叫 partial move：只移走部分欄位，其餘欄位仍可使用，已不完整的整份 `report` 則不能再當完整值交出去。本例只示範元件結果，沒有把它包裝成完整 CLI 的結束狀態政策：

```rust
--8<-- "examples/ch09-files/src/main.rs"
```

```bash
cargo run -p ch09-files --locked
```

教材目錄中 `.hidden/third.txt` 是 6 bytes，`first.txt` 與 `nested/second.txt` 各 5 bytes，結尾皆包含檔案中的換行。三項都能列出，最後 `掃描完整=true`。將 root 改成其中一個一般檔案時，則進入 `NotDirectory`，不是得到零項成功。

## 驗收 link 與非 Unicode 政策

7.8 的 `TempDir` 適合建立獨立材料，owner 保留到掃描完才清理。再補兩項 fixture API 的契約：

- `fs::create_dir(path) -> io::Result<()>` 建立最後一層目錄；父目錄必須已存在，不能當成一次建立所有上層。
- 在 `#[cfg(unix)]` 的測試中，`std::os::unix::fs::symlink(original, link) -> io::Result<()>` 建立指向 original 的 link；`std::os::unix::ffi::OsStringExt` 為 OsString 提供 `from_vec(Vec<u8>) -> OsString`，接手 Unix 原生檔名字節。`vec![255]` 不是合法 UTF-8，可在 Unix 作可控制的非 Unicode 材料。其他平台不編入這些測試。

測試另外直接建立三筆 iterator 材料：一般檔案的 Ok、不存在路徑的 Err、另一份一般檔案的 Ok，再把這個 Vec 交給 `collect_entries`。`WalkDir::new` 也能以一般檔案作 root，會產生一筆；不存在的 root 則交出 Err。因此可以驗證中間失敗有保留，且後面的成功項目仍有處理，沒有靠目前登入者的權限判定「一定不可讀」。這是收集元件的受控材料，不是把正式 `scan` 的 root 驗證關掉。

其他測試檢查隱藏子目錄、空檔／空目錄、root 檔案／不存在、root link 拒絕、內部檔案與目錄 link 不追蹤，以及非法 Unicode 檔名只造成一筆項目失敗。

```bash
cargo test -p ch09-files --test scan --locked
```

## 練習：以原生路徑查長度

把 `scan` 的成功記錄放到 `HashMap<PathBuf, u64>`，再用 `&Path` 查教材的 `nested/second.txt`，應得到 `Some(&5)`。

先想清楚兩件事：map 接手 `record.path`，不需要把它顯示再轉回來；`snapshot.len` 是另一個欄位中的 Copy 數值，沿用剛才的 partial move 關係，仍可取出。完成成功資料的 map 後，仍要回報 `issues.len()`，不能因查到 5 就認定整次掃描沒有失敗。

??? example "完整可執行答案"

    ```rust
    --8<-- "examples/ch09-files/examples/path_identity_solution.rs"
    ```

    ```bash
    cargo run -p ch09-files --example path_identity_solution --locked
    ```

現在有的是一般檔案候選、原生位置與每項觀察資料；政策略過與項目失敗也分開了。metadata 查詢成功仍不保證後續能開啟或完整讀取。下一節才讀 bytes，讓大小和摘要縮小比較工作，再用真正內容判定相同與否。

延伸閱讀：[walkdir 2.5 WalkDir](https://docs.rs/walkdir/2.5.0/walkdir/struct.WalkDir.html)、[Metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html)、[英文 Rust Cookbook：Directory Traversal](https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html)、[中文 Rusty Book 對應](https://rusty.course.rs/files/dir.html)。後兩份教材的某些短例會略過錯誤，本節保留它們，才有判斷掃描是否完整的依據。
