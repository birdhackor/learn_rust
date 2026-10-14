# 7.3 讀取文字檔，把失敗交給呼叫者

第 5 章已能解析文字、保留具體錯誤，再由呼叫者決定是否繼續。現在文字來自檔案：我們要讀出一份筆記，保留換行與中文；檔案不存在，或內容不是 UTF-8 時，也要得到可處理的失敗。

## 先準備真正的材料

本節使用 `ch07-io-text` package。從 repo 根目錄切到它的資料夾，**本節的全部指令都在這個資料夾執行**：

```bash
cd examples/ch07-cli/io-text
```

目前的工作目錄，就是程式解讀相對檔名的起點。下面的 `fixtures/note.txt` 因而指這個資料夾中的檔案，不是 Rust 原始碼檔案所在位置。7.5 會進一步處理路徑型別與工作目錄的 API；此刻先固定起點，讓讀取對象明確。

教材已提供三份材料，以及刻意未建立的檔名：

| 檔名 | 材料 | 本次要得到的結果 |
| --- | --- | --- |
| `fixtures/note.txt` | 下方的兩行文字，最後也有換行 | 取得完整 UTF-8 文字，長度 9 bytes |
| `fixtures/empty.txt` | 沒有內容的檔案 | 取得空 String，屬於成功 |
| `fixtures/not-utf8.bin` | 單一 byte 值 255，不是合法 UTF-8 | 取得文字讀取失敗 |
| `fixtures/missing.txt` | 不存在；請不要建立它 | 取得找不到檔案的失敗 |

`note.txt` 的內容是：

```text
--8<-- "examples/ch07-cli/io-text/fixtures/note.txt"
```

依 3.5 的 byte 規則，Rust 的四個 ASCII 字母加換行占 5 bytes，「中」加換行占 4 bytes，共 9。換行在 Rust 的字串 literal 中寫成 `\n`，所以完整內容可以表示為 `"Rust\n中\n"`。檔案沒有替我們分成 Rust 的字串清單；先讀回一份完整文字。

## read_to_string 的成功值與具體錯誤

**I/O** 是 input/output（輸入／輸出），此處指程式讀取或寫入外部資料。`std::fs` 是 filesystem（檔案系統）module，提供讀檔操作；`std::io` 提供 I/O 的共同結果與能力。

本節先使用 **`fs::read_to_string(filename)`**：傳入 `&str` 表示的檔名，它開啟並讀完整檔案。成功回傳 owned 的 `String`，內容必須是合法 UTF-8。

結果型別是 **`io::Result<String>`**，等同 `Result<String, io::Error>`。這延續 5.2 的 type alias：`io::Result` 替 I/O 工作固定錯誤型別為 **`io::Error`**，角括號只需要指定成功型別。它仍然是已教的 Ok／Err，沒有新的結果容器。

`io::Error` 保存這次 I/O 失敗；它已提供 Debug 與 Display，可以檢查或顯示。其 **`kind()`** method 借用錯誤，交回 **`io::ErrorKind`**：一個描述錯誤類別的 enum。本次先用兩個 variants：

- `NotFound`：要求的檔案或路徑部分不存在。
- `InvalidData`：資料不符合這項操作的要求；在文字讀取中，例如內容不是合法 UTF-8。

`ErrorKind` 還有其他類別，因此 match 要用已教的 `_` 處理其餘情況。不依作業系統產生的錯誤文字猜分類；類別供程式判斷，Display 訊息供人閱讀。

下面的 caller 每次處理一個結果，再繼續下一個檔名。成功時以 `{:?}` 顯示借來的文字，讓換行以 `\n` 看見，而非在終端機直接換行：

```rust
--8<-- "examples/ch07-cli/io-text/src/main.rs:read-example"
```

```bash
cargo run -p ch07-io-text --locked
```

```text
fixtures/note.txt：9 bytes
內容="Rust\n中\n"
fixtures/missing.txt：找不到檔案
fixtures/not-utf8.bin：內容不是合法 UTF-8
三個讀取結果都已處理
```

不存在與格式不合法都沒有成功文字可供讀取，但原因不同。Err 沒有自動停止程式；這個 caller 選擇逐筆報告。若檔案是合法的空文字，則得到 `Ok` 中的空 String，不能把它當成上述失敗。

## File 與 Read：保留開啟的檔案，追加到已有文字

剛才需要的是整份文字，所以一次呼叫就能完成。另一項需要是**把讀出的文字接在既有前綴之後**。先保留一個開啟的檔案，再指定文字要追加到哪裡，能把這兩份狀態分開。

**`std::fs::File`** 是開啟檔案的操作對象，管理和作業系統之間的檔案資源。`File::open(filename)` 以唯讀方式嘗試開啟檔案，回傳 `io::Result<File>`；取得 File 本身不代表已取得完整內容。正常離開 scope 時，File 會關閉它管理的檔案資源。

**`std::io::Read`** 是讀取來源的 trait，提供讀取資料的操作。File 已實作它；沿用 4.1 的規則，先 `use std::io::Read;`，才能以點號呼叫其 methods。

本次選用 **`file.read_to_string(&mut text)`**。它 mutable 借用 File，從目前位置讀到來源結尾，把合法 UTF-8 文字**追加**到給定的 String，不清除原內容。成功回傳 `io::Result<usize>` 中的**本次讀入 byte 數量**，不是 String 追加後的總長度。讀取會往後推進 File 的位置，因此 file 需要 mutable 存取。

helper 的回傳型別 `io::Result<(String, usize)>` 可以保留完整文字與新讀入量。這裡的 `?` 遇到 io::Error 就從 helper 回傳同一種錯誤；main 再決定顯示方式：

```rust
--8<-- "examples/ch07-cli/io-text/examples/file_handle.rs:handle-example"
```

```bash
cargo run -p ch07-io-text --example file_handle --locked
```

```text
新讀入=9 bytes
包含前綴=19 bytes
完整內容="筆記：\nRust\n中\n"
```

前綴的三個中文 scalar 加換行占 10 bytes；追加原檔案的 9 bytes，總長才是 19。兩個數字不同，正好確認 method 回報的是哪項工作。

若 UTF-8 無效，這個 method 回傳錯誤，給定的 String 不變。其他 I/O 錯誤則可能發生在已有部分讀入之後，不應把「取得 Err」推成所有外部工作從未發生。這個 helper 遇到失敗就回傳 Err，沒有把局部文字當成完整成功結果交出去。

## Write：把文字交給檔案出口

現在要把相同的 `"Rust\n中\n"` 存成 `note-output.txt`，再讀回確認。**`File::create(filename)`** 回傳 `io::Result<File>` 中可寫入的檔案：不存在就建立；**已存在就清空原內容**。因此範例只使用本 package 中專用的 `note-output.txt`，沒有拿教材輸入檔當輸出。

**`std::io::Write`** 是寫入出口的 trait。File 已提供它；先引入 Write，再用 5.3 的 **`write!`** 格式化文字到這個出口。這次 `write!` 的結果是 **`io::Result<()>`**：Ok 表示這次寫入完成，Err 保存 I/O 原因。它不固定寫到終端機，也不自動加換行；換行來自傳入文字。

5.3 的 `write!(f, ...)` 寫到 Formatter，回傳 fmt::Result；這裡寫到 File，回傳 io::Result。macro 的形式沿用，出口與失敗型別則由各自的能力決定。

```rust
--8<-- "examples/ch07-cli/io-text/examples/write_note.rs"
```

```bash
cargo run -p ch07-io-text --example write_note --locked
```

結果是 `重新讀取確認="Rust\n中\n"`。`save_note` 返回後，File 已正常結束，再重新開啟讀取檔案。寫入成功與重新讀取成功是兩次工作；後者可能單獨失敗，所以 caller 分別處理。

## 保存內容與錯誤分類的檢查

以下測試只讀教材提供的範例檔，沒有寫入共享檔案。Cargo 在執行 package 的測試時，以 package 根目錄作工作目錄，所以同樣的 `fixtures/...` 指到同一份材料。

ErrorKind 已有 Debug 與 PartialEq，能沿用 4.2 的 `assert_eq!` 比較類別。io::Error 本身不提供這項相等比較；因此先處理 Ok／Err，再比較 kind，不直接拿完整 I/O Result 做相等檢查。

```rust
--8<-- "examples/ch07-cli/io-text/src/main.rs:read-tests"
```

```rust
--8<-- "examples/ch07-cli/io-text/examples/file_handle.rs:handle-tests"
```

這次測試也放在 example target 裡，因此使用 **`--all-targets`** 讓 Cargo 執行各 target 的測試函式；單純的 `cargo test` 預設不會執行 example 裡的 tests。

```bash
cargo test -p ch07-io-text --all-targets --locked
```

三項測試分別檢查完整文字、失敗分類，以及新讀入量與含前綴總長不同。測試中的 expect 表達教材固定材料的成功預期；外部使用者的檔案仍由正常程式的 match 或 `?` 處理。

## 練習：兩份文字都讀到，才交回總長

寫 `total_file_bytes(first: &str, second: &str) -> io::Result<usize>`。依序呼叫本節的 fs::read_to_string，兩份都成功才回傳文字 byte 長度之和；任一步失敗就保留 io::Error，以 `?` 傳回 caller。

先自行驗證：note 加 empty 的結果為 9；note 加 not-utf8 在第二次讀取失敗，不能得到一個成功的部分長度。不要把失敗文字替換成空 String，否則 caller 會失去這個差別。

??? note "完整參考答案"

    ```rust
    --8<-- "examples/ch07-cli/io-text/examples/text_io_solution.rs"
    ```

    執行 `cargo run -p ch07-io-text --example text_io_solution --locked`。

目前我們把整份合法文字保存成 String。下一節改成固定大小的 byte buffer：讀取不一定一次填滿，也可能在一個 UTF-8 scalar 中間停下，要依回傳長度判斷哪些資料有效。

延伸閱讀：[英文 Rust Book：Reading a File](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)、[繁體 Rust Book：讀取檔案](https://rust-lang.tw/book-tw/ch12-02-reading-a-file.html)、[std：read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)、[Read](https://doc.rust-lang.org/std/io/trait.Read.html)、[Write](https://doc.rust-lang.org/std/io/trait.Write.html)。
