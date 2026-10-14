# 7.5 原生路徑與 CLI：檔名、資料、診斷各有自己的表示

前兩節使用固定檔名讀取筆記。現在要讓呼叫程式的人指定檔案，成功就交出原文字，失敗則回報原因與結束狀態。這份需要還包含一項區別：**檔名是作業系統接受的路徑，檔案內容才是這個工具要求的 UTF-8 文字**。

本節使用 `ch07-path-cli` package；從 repo 根目錄執行 `cd examples/ch07-cli/path-cli`，以下指令都在這個資料夾執行。先把路徑與原生字串讀清楚，再接上 6.5 已教的 Iterator 與 7.3–7.4 的 I/O。

## PathBuf 擁有路徑，&Path 借用路徑

**path（路徑）** 指向檔案系統中的位置，可能包含目錄與檔名。`std::path` 的 **`Path`** 表示路徑內容，本節透過 `&Path` 借用它；**`PathBuf`** 擁有這份路徑資料，可以在 mutable 存取下修改。它們的借用／owned 分工，和 str／String 相似。

路徑資料不是已開啟的 File。組好 `notes/2026/note.txt` 這份路徑，沒有因此建立目錄、驗證檔案存在或讀出檔案內容；真正開啟與讀取仍由 I/O 操作完成。

先選用四個操作：

| 操作 | 接收與交回 | 在本節的用途 |
| --- | --- | --- |
| `Path::new("notes")` | 從給定文字借出 `&Path` | 借用已有的目錄表示，不建立 owned 的複本。 |
| `path.join("2026")` | shared 借用 Path，交回新的 PathBuf | 保存加入相對路徑後的結果，原 Path 不變。 |
| `filename.push("note.txt")` | mutable 操作 PathBuf，回傳 unit | 將相對的檔名接到已有路徑。 |
| `filename.as_path()` | shared 借用 PathBuf，交回 `&Path` | 交給只需讀路徑的工作，保留 caller 的 owner。 |

join 與 push 使用平台的路徑規則安排分隔符號。本例加入的都是相對片段；傳入完整的絕對路徑時，可以替換原路徑，不能把它們當成一律追加文字的 String 操作。

Path／PathBuf 提供 Debug 與 PartialEq，可以檢查或比較路徑表示。比較依路徑組成進行，沒有查詢檔案系統證明兩份路徑指同一個真實檔案。

顯示時選用 **`path.display()`**：它交回供格式化使用的 `std::path::Display` 物件，實作 5.3 的 Display trait；它借用原路徑，**不是產生一份 String**。若路徑不能完整表示成 Unicode，這項顯示可能有失真，所以供人閱讀，不拿顯示文字重新建立檔名或作唯一識別。

若確實需要 UTF-8 文字，**`path.to_str()`** 回傳 `Option<&str>`：可表示就 Some，否則 None。None 不代表路徑資料本身不存在或一定無效，只表示不能借成這份文字表示。

```rust
--8<-- "examples/ch07-cli/path-cli/examples/path_types.rs"
```

```bash
cargo run -p ch07-path-cli --example path_types --locked
```

原目錄仍是 notes，新的路徑在這個平台上含 notes、2026、note.txt 三段。這份由合法文字建立的路徑可轉成 &str；操作結果沒有查詢實際檔案。

## OsString／OsStr 保留作業系統的字串資料

**OS** 是 operating system（作業系統）。原生檔名不保證能用 UTF-8 表示，因此標準庫另有 **`OsString`** 保存 owned 的 OS 字串，**`&OsStr`** 借用其內容；型別位於 `std::ffi` module。ffi 是 foreign function interface（外部函式介面），這個 module 提供和外部系統交換資料的型別；本節先用其中的原生字串。

OsString 沒有 Copy；交出去會 move，需要獨立副本時則已有 Clone。這些型別也有 Debug、PartialEq，能檢查或比較原生內容。不要因為名稱含 String，就把它當成必定合法 UTF-8 的 String。

本節選用的轉換分成兩種目的：

| 操作 | ownership 與結果 | 保留或檢查什麼 |
| --- | --- | --- |
| `OsString::from("中文筆記.txt")` | 從 `&str` 建立 owned OsString | 合法文字一定能成為原生字串。 |
| `native.as_os_str()` | 借出 `&OsStr` | 沒有接手 native。 |
| `os_text.to_str()` | 回傳 `Option<&str>` | 成功則借出 UTF-8 表示，失敗沒有文字 reference。 |
| `native.into_string()` | 接手 OsString，回傳 `Result<String, OsString>` | 成功交回 owned 文字；失敗把原 OsString 的 ownership 交回 Err。 |
| `PathBuf::from(native)` | 接手 OsString，交回 owned PathBuf | 保留原生資料成為路徑，不要求先轉成 UTF-8。 |
| `path.as_os_str()` | 從 `&Path` 借出 `&OsStr` | 需要原生字串的 API 可以使用，仍不轉成 UTF-8。 |

into_string 的 Err 中是**原生資料本身**，不是新增的一段錯誤訊息；caller 仍可以保存它或轉成 PathBuf。下面先借用再交成路徑，最後另用一份固定 OsString 觀察 owned 文字轉換：

```rust
--8<-- "examples/ch07-cli/path-cli/examples/os_strings.rs"
```

執行 `cargo run -p ch07-path-cli --example os_strings --locked`。中文檔名原本來自合法文字，所以本例能借成 UTF-8。實際從 OS 取得的字串則需要處理 None／Err，不能只依這個成功例保證所有原生檔名都能轉換。

## 工作目錄與真正的路徑讀取

**`std::env::current_dir()`** 取得目前行程的工作目錄，回傳 `io::Result<PathBuf>`；不是 crate 原始碼的位置，也不是一定指向執行檔所在目錄。caller 可以借用這份 owned 路徑，再 join 相對檔名。

7.3 的 fs::read_to_string 不只接受文字檔名，也接受 **`&Path`**。它的輸入需要能借成 Path，這是 5.5 的 `AsRef<Path>` 關係；本節明確傳 `filename.as_path()`，不靠把原生路徑先轉成 String。

```rust
--8<-- "examples/ch07-cli/path-cli/examples/working_directory.rs"
```

執行 `cargo run -p ch07-path-cli --example working_directory --locked`。工作目錄依你的 repo 存放位置而不同，讀到的教材筆記仍是 `"Rust\n中\n"`。join 不自行清除 `..`；真正讀檔時才由檔案系統解讀這份位置。

## 用 args_os 接收原生 arguments

**CLI** 是 command-line interface（命令列介面），也就是透過命令列參數、輸出與結束狀態使用程式的方式。arguments 是啟動程式時交給它的參數資料。本次只接受一個檔名；少了或多了，先回報用法錯誤，不開始讀檔。

**`env::args_os()`** 建立標準型別 `ArgsOs` 的 iterator，其 **Item 是 owned OsString**。6.5 的 next 每次取得 Some 中的一筆，結束是 None；不必先收集成 Vec，再冒著索引越界去取參數。

第一筆按慣例是程式名稱，這個工具略過它，再讀真正的檔名。它可以是任意啟動名稱，不能把它當成保證存在的執行檔路徑。args_os 保留原生資料；如果使用要求 Unicode 的 args，遇到不合法 Unicode 的參數可能 panic，因此本章選擇前者。

先以具體 enum 保存兩種用法失敗；Debug／PartialEq／Display 都沿用第 4、5 章：

```rust
--8<-- "examples/ch07-cli/path-cli/src/lib.rs:argument-error"
```

library 的 parse_filename 不自行向 OS 取資料。它接收 **`I: IntoIterator<Item = OsString>`**，讓呼叫者交入完整參數序列，也能以已教的 owned array 驗證。IntoIterator 建立工作，next 推進狀態，取得的 OsString move 成 PathBuf。

`let _program_name = arguments.next();` 仍有執行 next，因此略過第一筆。變數名稱以底線開頭表示刻意不再使用這份結果；不是省略了這次推進。

```rust
--8<-- "examples/ch07-cli/path-cli/src/lib.rs:argument-parser"
```

只有略過名稱後剛好一個檔名，才會取得 Ok 中的 PathBuf。沒有先把檔名轉成 UTF-8，再決定能否讀檔。

### 先驗證 library 的參數規則

PathBuf 與 ArgumentError 都已提供 PartialEq／Debug，所以 Result 的比較符合 5.1 已教的條件。以下測試保存合法中文檔名、少參數與多參數三個結果：

```rust
--8<-- "examples/ch07-cli/path-cli/src/lib.rs:argument-tests"
```

執行 `cargo test -p ch07-path-cli --locked`，這兩項測試沒有啟動外部 process。實際 CLI 的 stdout／stderr／結束狀態，要在接上 main 之後另驗證。

## stdout、stderr 與正常返回的 ExitCode

行程有兩個常用輸出出口：**stdout（standard output，標準輸出）** 交出工具的資料；**stderr（standard error，標準錯誤）** 交出診斷訊息。終端機通常都顯示兩者，但 caller 可以分別保存；本次成功的筆記內容只進 stdout。

`io::stdout()` 交回 Stdout，`io::stderr()` 交回 Stderr。它們是操作對應標準出口的物件，都提供 7.3–7.4 的 Write，可以以 mutable binding 呼叫 write!／write_all 並處理 io::Error。它們沒有替程式選擇該輸出哪些內容。

Stdout 可能先把資料留在行程共用的 buffer；連到終端機時通常以換行為推送邊界。這次要保留原筆記，不能假定最後一定有換行。因此 write_all 成功後，還要呼叫 7.4 已教的 flush，檢查緩衝資料能否交給標準出口，再選成功狀態。正常結束時的自動清理不能替 main 回報這一步的錯誤；flush 成功也沒有承諾永久儲存。Stderr 預設沒有這份緩衝，下面仍直接檢查每次診斷寫入的結果。

**exit status（結束狀態）** 是程式結束時交給啟動者的結果。Rust 的 **`std::process::ExitCode`** 可以由 main 正常回傳：

- `ExitCode::SUCCESS` 是標準成功值，`ExitCode::FAILURE` 是標準失敗值。這些大寫名稱是型別提供的既定值，不需加小括號。
- `ExitCode::from(status)` 接收 u8，依 From 建立指定數字的結果；本章用 2 表示參數用法不符。
- main 寫成 `fn main() -> ExitCode`，各返回路徑都交出這個型別。正常離開 scope，仍會完成值的清理，再將結果交給啟動者。

`process::exit` 是立即停止目前行程的另一個 API，會略過目前 scope 的正常清理；本章使用 main 回傳 ExitCode，讓之後臨時目錄等資源也能依正常 scope 結束。

**`writeln!`** 沿用 write! 的出口與格式字串，額外加上一個換行；寫入 Stderr 時同樣回傳 io::Result<()>。先建立報告函式，成功就交回希望的狀態；連診斷出口都失敗，就交回標準失敗。沒有遞迴要求同一個已失敗的 stderr 再報告自己：

```rust
--8<-- "examples/ch07-cli/path-cli/src/main.rs:streams"
```

接著讓 main 取得 arguments、呼叫 library，再用 &Path 讀取。format／to_string 都沿用 5.3–5.5，Path 的 display 只進診斷；操作真正使用的是原生 Path。先 write_all 原文字的 bytes，再 flush；兩步都成功才回傳成功狀態，沒有加標題或額外換行：

```rust
--8<-- "examples/ch07-cli/path-cli/src/main.rs:cli-main"
```

## 實際觀察資料、診斷與狀態

Cargo 指令的 **`--`** 表示後面的 arguments 交給我們的程式，而非 Cargo。**`--quiet`** 則減少 Cargo 自己的例行建置／執行訊息，程式的 stdout／stderr 仍照常存在。

```bash
cargo run --quiet -p ch07-path-cli --locked -- ../io-text/fixtures/note.txt
```

終端機只有兩行筆記 Rust 與中。沒有檔名、傳入兩個檔名、指定不存在的檔案，或指定 not-utf8.bin，都走診斷路線；最後一項是內容不符合 UTF-8，與檔名能否轉成 UTF-8 分開。

以下是 Bash 中的觀察方式：**`>`** 把 stdout 存到指定檔案，**`2>`** 把 stderr 存到另一個檔案，既有輸出檔會被清空；本次只使用兩個專用 captured 檔案。`echo $?` 顯示上一個指令的結束狀態，要立即放在 run 後面。

```bash
cargo run --quiet -p ch07-path-cli --locked -- ../io-text/fixtures/note.txt > captured-note.txt 2> captured-errors.txt
echo $?
```

本教材的 Linux 環境中，成功顯示 0；captured-note.txt 是原筆記，captured-errors.txt 是空檔。改為不存在的檔名，正常失敗顯示 1，資料檔空白而診斷檔有原因；未提供檔名則是 2。若以 PowerShell 操作，保存兩個出口後用 `echo $LASTEXITCODE` 觀察上一個原生程式的狀態。

資料、診斷與狀態各回答一個問題：工具產出什麼？失敗為什麼？caller 能否把這次當成成功？把三者分開，下一個程式才不會把錯誤訊息誤當筆記內容。

## 練習：先接受兩個原生檔名

先定義 PairError，包含 MissingFilename 與 TooManyArguments 兩個 variants，再寫 `parse_pair<I: IntoIterator<Item = OsString>>(input: I) -> Result<(PathBuf, PathBuf), PairError>`。同樣略過程式名稱，改為剛好接收兩個檔名；不夠兩個回 MissingFilename，再多一個回 TooManyArguments。不要轉成 String，也不要開檔案。

這份練習以 match 核對錯誤分類，不另外增加顯示介面。先驗證合法中文名稱、少第二個、以及多第三個；正確時由 caller 分別取得兩個 owned PathBuf。

??? note "完整參考答案"

    ```rust
    --8<-- "examples/ch07-cli/path-cli/examples/argument_pair_solution.rs"
    ```

    執行 `cargo run -p ch07-path-cli --example argument_pair_solution --locked`。

我們已經能完整處理一個原生檔名與 CLI 輸出。下一節加入第三方參數解析 library，處理選項與說明；仍先看它接受什麼、交回什麼，以及何時真的是用法錯誤。

延伸閱讀：[英文 Rust Book：Arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)、[繁體對應章](https://rust-lang.tw/book-tw/ch12-01-accepting-command-line-arguments.html)、[std：Path](https://doc.rust-lang.org/std/path/struct.Path.html)、[OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html)、[args_os](https://doc.rust-lang.org/std/env/fn.args_os.html)、[ExitCode](https://doc.rust-lang.org/std/process/struct.ExitCode.html)。
