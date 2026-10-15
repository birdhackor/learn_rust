# 8.5 FromStr、AsRef 與 ToOwned：解析文字，或取得另一種表示

第 8.4 節的來源已經是 i32。現在輸入換成 `"3"`、`"abc"`、`"101"`；要取得 Count，必須先把文字解析成整數，再檢查0到100的件數規則。兩個步驟的失敗要保留不同原因。

本節也分開兩個常見需要：只借出現有文字，與建立可獨立保存的 owned 產物。它們都可能被稱作「轉換」，卻沒有在做同一項工作。

## FromStr 從文字解析一個值

**`FromStr`** 是標準的解析 trait，名稱中的 str 指文字。它要求 `from_str(text: &str) -> Result<Self, Self::Err>`：借讀文字，成功時交出目的型別的值，失敗時交出實作選定的錯誤。Err 是第 6.4 節的 associated type，與 TryFrom 的 Error 同樣由 impl 指定，只是名稱不同。

標準 i32 已實作 FromStr。本例使用十進位整數文字，至少需要一位數字，可在前面帶 `+` 或 `-`；不能有前後空白、其他字元，數字也必須能由 i32 表示。因此 `"3"`、`"+3"` 成功；空字串、`"abc"`、`"3 "`、超過 i32 上限的 `"2147483648"` 都失敗。

它的錯誤型別是 **`std::num::ParseIntError`**，保存整數解析失敗原因，已提供 Debug、Display、PartialEq。所以下面能顯示錯誤，也能比較 `Result<i32, ParseIntError>`；現在不需要自己建構錯誤或讀取其內部欄位。

直接使用 `i32::from_str("3")` 時，以 `use std::str::FromStr;` 引入 trait。文字的 **`parse::<i32>()`** 則是 str 提供的方法，依指定的目的型別呼叫其 FromStr，同樣交回 `Result<i32, ParseIntError>`。`::<i32>` 是第 5.1 節的 turbofish，這次指定解析目標。只用 parse 時，不必為了這個方法另引入 FromStr 名稱。

下面是完整自足程式：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/integer_parsing.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example integer_parsing --locked
```

```text
兩種寫法都取得整數 3
解析 "abc" 失敗：invalid digit found in string
解析 "3 " 失敗：invalid digit found in string
解析 "2147483648" 失敗：number too large to fit in target type
```

前兩項 assertion 取得 Ok(3)。後三筆雖失敗，呼叫者仍逐筆回報；這些原本可能出現的解析錯誤沒有被改成 panic。

## 用一個 enum 保存兩種步驟的錯誤

`"abc"` 還沒取得整數，屬於 ParseIntError；`"101"` 能取得整數，但件數驗證產生 CountError。**`ParseCountError`** 因而需要兩種 variant：

| variant | 表示的步驟 | 保存的原原因 |
| --- | --- | --- |
| `Integer(ParseIntError)` | 整數解析失敗 | 標準整數錯誤 |
| `Range(CountError)` | 件數規則不接受已取得的整數 | 本章件數錯誤 |

兩者都保存原錯誤值，讓呼叫者能用 pattern 分類，不是事後猜測訊息文字。Display 在原原因前加上步驟說明；impl 裡的 Self 代表 ParseCountError，所以 `Self::Integer` 就是其 Integer variant。

fmt 借讀 self，pattern 中的 error 是原錯誤的 shared reference；兩種原錯誤已有 Display，依第 8.3 節 reference 的顯示契約，可直接借讀來寫訊息：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/parsing/parsing/mod.rs:parse-count-error"
```

第 8.4 節的 From 也能封裝錯誤。把 ParseIntError、CountError 放進對應 variant，一定能保留原原因，不需新的驗證。下面兩份實作以值接收原錯誤，把其 ownership 交給外層錯誤：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/parsing/parsing/mod.rs:parse-error-from"
```

這個 enum 只收進本項解析工作會遇到的兩個來源，沒有替其他工作任意合併所有失敗。

## 為 Count 實作 FromStr

`type Err = ParseCountError;` 選定解析失敗的型別，所以這裡的回傳型別實際是 `Result<Count, ParseCountError>`。先解析 i32，失敗就以 ParseCountError::from 封裝並依第 2.2 節早退；成功後才呼叫 Count::try_from 驗證件數。

```rust
--8<-- "examples/intro-workspace/error-stages/examples/parsing/parsing/mod.rs:count-from-str"
```

Count 只有 owned 的 i32，沒有 reference 欄位；此實作的成功結果不借用輸入文字。文字在這裡只用來取得數字，沒有把某份輸出reference的 lifetime 連回 text。

完整呼叫者使用 Count::from_str 與 `parse::<Count>` 取得相同件數，再逐筆處理兩種失敗。String 輸入先以已教的 as_str 借出文字；parse 不接手原 String，所以之後仍能使用：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/parsing/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example parsing --locked
```

```text
兩種寫法都取得件數 3
0：有效件數 0
100：有效件數 100
abc：無法解析整數：invalid digit found in string
-1：件數不符規則：件數 -1 不能為負數
101：件數不符規則：件數 101 超過批次上限100
原文字仍可使用：+3
```

訊息前綴對應兩個實際步驟：abc沒有取得整數，−1／101則已有整數、但件數不合規則。CountError 的完整句子是給使用者看的說明，不是 i32 的整數格式；Display 與 FromStr 都處理文字，不代表兩者必定能往返。

## AsRef 借出共同的 view

若只想知道文字的 byte 長度，就不需要先解析成數字。**`AsRef<T>`** 描述便宜、一定成功的借用式轉換；`as_ref(&self)` 借讀接收者，交回 `&T`。`AsRef<str>` 因而交回 &str，不建立新 String。

String 與 &str 都有標準的 `AsRef<str>` 能力。因此 `text_bytes<T: AsRef<str>>(text: &T) -> usize` 以第 6.1 節的 bound 要求共同操作，先借到文字，再呼叫 len 取得 byte 長度。

對 String 變數 owned，傳 `&owned`；對已是 &str 的 borrowed，仍傳 `&borrowed`。後者是 **`&&str`**，也就是再借讀那份 reference 值，沒有接手它指向的文字。兩種來源都保留在呼叫者。

AsRef 也能只投影一個欄位。下面 Label 的完整身分由 id、text 兩欄決定；PartialEq／Eq／Hash 沿用第 6.6 節的 derive 規則。它的 `AsRef<str>` 只借出 text。`as_ref(&self) -> &str` 的輸出依第 7.4 節 receiver elision 連到這次 self 借用，因此結果使用時仍須能借讀 Label。

```rust
--8<-- "examples/intro-workspace/error-stages/examples/as_ref.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example as_ref --locked
```

```text
&str 與 String 都有 4 bytes
String 仍在：Rust
Label 身分不同，借出的標籤文字相同
```

兩個 Label 的 id 是1與2，所以完整值不同；借出的 Rust 文字卻相等。這正是 AsRef 與第 6.5 節 `Borrow<str>` 的差別：Borrow 作鍵查詢時要求借用表示保留 Eq／Hash 的同一個身分。只借 text 會把這兩個不同 Label 當成同一個文字鍵，因此不能把此欄位投影當成符合那項承諾的 Borrow 實作。

## ToString 與 ToOwned 建立可獨立保存的結果

**`ToString`** 的 `to_string(&self)` 借讀值，交回owned String。標準函式庫替有Display的型別提供它，不必自己實作ToString。這與 `format!("{error}")` 一樣保存其Display文字，單一值不必再寫格式字串。

**`ToOwned`** 從借來的資料建立owned產物。associated type **`Owned`** 指定產物型別；本節只使用以下兩份標準契約：

| 借來的資料 | to_owned產物 | 本次複製的內容 |
| --- | --- | --- |
| `text: &str` | String | 把文字內容放進新String |
| `values: &[i32]` | `Vec<i32>` | 把數字元素放進新Vec |

這兩種產物不再借用原資料，可獨立保存、修改；原資料也不因此失去ownership。第 6.3 節的Clone則從借讀T取得另一個T，行為仍依型別而定。只複製&str的reference資訊，仍借原文字，與此處的str→String不同。

AsRef、ToString、ToOwned都在prelude；範例仍明寫標準路徑讓來源可追蹤。下面完整根檔案先保存錯誤的Display文字，再分別修改owned文字、owned數字清單，原資料保持原內容：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/owned_text/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example owned_text --locked
```

```text
保存顯示文字：件數 -1 不能為負數
原錯誤仍可顯示：件數 -1 不能為負數
原文字：Rust；owned 複本：Rust 教學
原 array 有 2 筆；owned Vec 有 3 筆
```

兩份新增產物與來源可各自使用，不只是多了一個reference。這些具體契約也沒有承諾任意型別的to_owned都會移除其內含借用；要看產物實際保存什麼。

## 完整來源與解析政策測試

本頁全部targets仍使用no-lib的intro-error-stages，完整member設定：

```toml
--8<-- "examples/intro-workspace/error-stages/Cargo.toml"
```

integer_parsing與as_ref已是完整單檔；parsing的完整main也已列出。它的parsing/mod.rs包含四項測試：整數格式與合法邊界、整數失敗、件數失敗、來源文字結束後仍可使用的owned Count。

測試中的 `for (text, expected) in ...` 沿用第 4.3 節的tuple pattern，把每次取得的tuple拆成兩個名稱。`other => ...` 是名稱pattern，接住其餘整個結果，供失敗時Debug顯示，與 `_` 略過不同。最後一項用已教的 `let count = { ... };`，由內層block尾端交出owned Count，沒有無初值宣告：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/parsing/parsing/mod.rs"
```

以下是parsing target實際載入的其餘三份完整helper；來源閉包沒有propagation或?：

??? note "parsing target 的完整其餘來源"

    `conversions/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing/conversions/mod.rs"
    ```

    `policy/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing/policy/mod.rs"
    ```

    `formatting/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing/formatting/mod.rs"
    ```

owned_text只需要policy、formatting，它的完整兩份快照如下，不載解析：

??? note "owned_text target 的完整來源"

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/owned_text/policy/mod.rs"
    ```

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/owned_text/formatting/mod.rs"
    ```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example parsing --locked
```

parsing共十項測試：policy兩項、formatting一項、conversions三項、本節四項。`"2147483648"`是整數不能表示，`"101"`是整數成功但件數不合法，兩者必須留在不同variant。

## 練習與完整答案

寫 `parse_count<T: AsRef<str>>(input: &T) -> Result<Count, ParseCountError>`，只借讀不同來源，交回Count的解析結果。讓&str與String都保存`"100"`，確認結果相等、原String仍可用；再解析`"101"`，確認得到件數規則失敗。

這裡只需把input.as_ref交給已教的parse，不需要下一節的?。完整根檔案如下，固定成功文字的expect仍明說保證：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/parsing_answer/main.rs"
```

??? note "答案實際載入的全部四份完整 helper"

    `parsing/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing_answer/parsing/mod.rs"
    ```

    `conversions/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing_answer/conversions/mod.rs"
    ```

    `policy/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing_answer/policy/mod.rs"
    ```

    `formatting/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/parsing_answer/formatting/mod.rs"
    ```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example parsing_answer --locked
```

```text
兩種來源都取得件數 100
原 String 仍在：100
失敗原因：件數不符規則：件數 101 超過批次上限100
```

現在能依需要選擇：解析要保留失敗原因；AsRef只借出現有view；ToString與本例ToOwned建立可保存的owned結果。下一節用?縮短傳回失敗的程式，保留呼叫者決定後續的責任。

[下一節：?與錯誤傳遞](08-06-lesson.md)
