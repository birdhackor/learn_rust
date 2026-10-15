# 6.2 Debug、PartialEq 與 derive：顯示及比較整筆資料

前面顯示訂單時，總是分別選出金額、數量。若要一次檢查整筆摘要，或判斷兩份摘要是否相等，就需要型別提供相應能力。標準函式庫已有 `Debug` 與 `PartialEq`；這次先選定我們希望它們對欄位做什麼，再請編譯器產生實作。

## Debug 把資料顯示給開發者檢查

**`Debug`** 提供供開發者檢查的文字表示。`println!("{summary:?}")` 中的 **`:?`** 要求用 `Debug`，原來的 `{summary}` 不會自動切換到這種顯示方式。這次格式化只借讀 summary，沒有取得它的 ownership；顯示後仍能讀取它的欄位。

對兩個整數欄位的摘要，我們希望一起看見型別名、欄位名與各自的值。寫在型別前面的 **`#[derive(Debug)]`**，就是要求編譯器替這個型別產生 `Debug` 實作。`#[...]` 延續第 2.7 節的 attribute 語法；`derive(...)` 列出這次希望產生的 trait。

產生這份實作時，每個欄位也要有 `Debug`；`i32` 已有。因此下面可以逐欄顯示，再合成一份完整摘要：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/debug.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example debug --locked
```

```text
整筆資料=OrderSummary { amount: 360, count: 3 }
```

這種輸出方便檢查當時的值；其具體格式可以隨 Rust 版本或實作改變。將來要規定存檔或對外輸出的格式，不能把這個外觀當固定協定。

若移除 attribute，欄位仍各自能顯示，但整個型別沒有 `Debug`。下面是完整的預期編譯失敗例，會得到 **E0277**：

```rust
--8<-- "examples/intro-workspace/trait-stages/fixtures/compile-fail/not_debug.rs"
```

## PartialEq 依我們選的欄位規則比較

兩筆摘要的金額相同、數量不同，這裡應算不同摘要。我們選擇「全部欄位都相等，整筆才相等」。

**`PartialEq`** 提供 `==` 與 `!=` 的相等比較能力，結果是 `bool`。數字本來就有相應實作；自訂 struct 不會自動取得它。`#[derive(PartialEq)]` 產生逐欄比較，因此每個欄位也必須有 `PartialEq`。

對 enum，derived 比較先要求同一個 variant，再比較該 variant 攜帶的全部值。例如 `Waiting` 與 `Ready` 不同；`Ready { count: 3 }` 與 `Ready { count: 4 }` 也不同。

一個 derive attribute 可以用逗號列出多個 trait。下面同時產生 `Debug` 與 `PartialEq`，因此 `assert_eq!` 可以比較整筆資料：前者供 assertion 失敗時顯示左右值，後者判斷是否相等。這些比較會借讀左右兩邊，不會把摘要 move 走，比較後仍可使用。

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/equality.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example equality --locked
```

```text
相同欄位=true
同金額、不同數量=false
等待與可出貨相等=false
相同形態、不同數量=false
```

這些材料分別只改數量、改 variant，再改同 variant 的數量，讓兩種逐欄規則都可見。若應用程式希望忽略某個欄位，derive 的規則就不符合需求；編譯器不會從欄位名稱猜出什麼才算「相同」。

## 標準文字也能顯示與比較

接著要讓摘要帶有文字標籤。先把後面會用到的標準規則說清楚：`String` 已提供 `Debug` 與 `PartialEq`，比較的是文字內容；兩份獨立建立、內容同為 `Rust` 的 String 會相等。`&str` 也能使用這兩項能力，仍以所借文字的內容判斷相等。

`Debug` 顯示文字時會包含引號，與原來 `{text}` 直接顯示內容的方式不同。下面用已教的 `String::new`、`push_str`、`as_str` 建立兩份文字及一份視圖，再比較它們：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/standard_equality.rs:text"
```

兩項 assertion 都成功；輸出中 owned 與借用都顯示 `"Rust"`，比較後 first 仍可以借讀。比較相同不代表兩份 owned String 是同一個物件，也不會把其中一份交給另一份。

## reference 比較借到的值

`usize` 和前面的整數一樣，已有 `Debug` 與 `PartialEq`。shared reference 在被借型別有相應能力時，也能以 `Debug` 顯示被借值、以 `PartialEq` 比較被借值。因此兩份分別借自不同變數、內容同為 2 的 `&usize`，比較仍相等：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/standard_equality.rs:references"
```

輸出是 `兩份借用的內容相等=true`。這裡的相等回答「借到的值是否相等」，沒有在測量兩份 reference 是否保存相同位置。

## Option：先比較形態，再比較內容

第 5.2 節的 `Option<T>` 是 enum。標準函式庫在 `T` 有 `PartialEq` 時，也提供 `Option<T>` 的相等比較：兩個 `None` 相等；`Some` 與 `None` 不相等；兩個 `Some` 要看裡面的值是否相等。`T` 有 `Debug` 時，整份 Option 也能 Debug 顯示，例如 `Some(2)` 或 `None`。

`Option<usize>` 因而能直接比較。`Option<&usize>` 也符合條件：成功時 reference 比較它借到的數值，缺值時仍是 None。下面的 `borrowed` 與另一份 `Some(other_ref)`，雖然 reference 來源不同，借到的值都為 2，所以相等：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/standard_equality.rs:options"
```

這讓後面查詢「有沒有取得值、取得的是不是 2」的測試能直接比較整份結果。它沒有把缺值換成零，也沒有略過缺值時的檢查。

### 完整的標準型別範例

下面是以上三段的完整程式，沒有其他 helper：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/standard_equality.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example standard_equality --locked
```

```text
owned="Rust"，借用="Rust"
比較後仍可讀=Rust
兩份借用的內容相等=true
有值=Some(2)，缺值=None
借到的結果=Some(2)
```

本節每個 example 都是獨立完整檔，不載入 library。共同的完整 member manifest 如下：

```toml
--8<-- "examples/intro-workspace/trait-stages/Cargo.toml"
```

## 練習與完整答案

定義有 `label: String`、`amount: i32` 的摘要，產生 `Debug`、`PartialEq`。用已教的文字建立方式各自建立兩筆「訂單A」、金額280的摘要，先確認整筆相等；再只把第二筆金額增加20。

檢查原金額仍280、新金額300、兩份標籤文字相等，整筆摘要卻不相等。最後以 Debug 顯示兩筆。這一題還沒使用下一節的複製方法，兩份文字都由你明確建立。

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/equality_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example equality_answer --locked
```

```text
原摘要=LabelledSummary { label: "訂單A", amount: 280 }
改金額後=LabelledSummary { label: "訂單A", amount: 300 }
```

標籤也有 Debug 與 PartialEq，才能讓整份摘要產生這兩項實作；改一個欄位後，整筆相等的結果就改變。下一節再保留原摘要，明確建立另一份可修改的值。

[下一節：Clone 與 Copy](06-03-lesson.md)
