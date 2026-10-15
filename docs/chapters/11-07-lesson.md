# 11.7 以 Error 介面借讀原因，形成完整報告

第 8 章已把整數解析與件數驗證的失敗保存在 enum，caller 可以 match 分類。現在希望報告器不用知道每個 module 的 variant，仍能顯示「哪項工作失敗」，再沿保存的下層原因繼續說明。

Result 本來就沒有要求 E 實作某個 trait。這裡新增 **`std::error::Error`**，是為了共同的顯示與原因借用介面，而不是讓 Result 開始能保存錯誤。

## Error 的要求與預設 source

Error 要求 **`Debug + Display`**；這是第 11.6 節的 supertrait 關係，兩項要求一起成立，因此借用 Error 介面也能顯示錯誤。

它提供 **`source(&self) -> Option<&(dyn Error + 'static)>`**，借出下一層原因：Some 有原因 reference，None 表示沒有下一層。回傳 reference 依第 7.4 節連到 self；裡面的 static 限制原因型別所含借用，兩者要分開。

trait 也可以替 method 提供 body，稱為**預設實作**。Error 的 source 預設回傳 None，所以沒有下層原因時，可以寫空的 `impl Error for CountError {}`；不是缺少必要實作。

先沿用第 8 章0到100的件數規則與 Display，新增 Error 能力。下面這個階段只有 leaf error，不載入後面的原因鏈：

```toml
--8<-- "examples/intro-workspace/type-stages/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_leaf/errors/mod.rs"
```

caller 引入 Error，才能依 trait method 呼叫 source；完整根檔案如下：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_leaf/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example error_leaf --locked
```

```text
Debug檢查=Negative(-1)
Display訊息=件數 -1 不能為負數
這個錯誤已是最後原因
```

None 沒有憑空增加原因。若只有 Display、沒有 Debug，Error 的要求就不成立；下面完整負例與 `.expected` 在獨立專案執行 `cargo check` 得到 **E0277**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/error_without_debug.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/error_without_debug.expected"
```

## 各層保存自己的資料，source 借出原原因

ParseCountError 延續第 8.5 節的 Integer／Range，分別擁有 ParseIntError 或 CountError。標準 ParseIntError 也有 Error；因此 **source method** 可以對 `&self` 作 match，將兩種欄位 reference unsize 成共同的 `&dyn Error`。

這次 Display 只說目前這層工作，例如「解析後的件數不合法」，下層原數字留在 source。若外層 Display 已完整顯示原因，再沿 source 顯示一次，就會重複；這是報告器的分工選擇，沒有移除原錯誤。

再用 ReadCountError 保存輸入 String 與 ParseCountError。read_count 失敗時以第 8.5 節的 to_owned 保存輸入，將 cause move 進欄位；這裡只處理傳入文字，沒有檔案 I/O。

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_chain/errors/mod.rs:parse-error"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_chain/errors/mod.rs:read-error"
```

圖對應 abc 的三層資料。每個 source 箭頭借讀下一層，不複製或接手它：

![ReadCountError 借出 ParseCountError，再借出 ParseIntError，最後 source 為 None](../assets/types-v3/error-chain.svg)

## 同一個 report 走訪有限原因鏈

report 收到 `&dyn Error`，先 format 外層文字，再用 loop／match 追蹤 source。Some 時加入這層 Display，再取下一層；None 時交回 owned String。這些例子的原因鏈都向下前進、最後結束，沒有循環回自己。

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_chain/errors/mod.rs:report"
```

以下是這一步的**完整** helper，包含 leaf、解析、輸入 context、report；沒有 borrowed context 或 boxed boundary：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_chain/errors/mod.rs"
```

完整 caller 使用同頁 manifest：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_chain/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example error_chain --locked
```

```text
成功件數=3
處理失敗：讀取件數文字「abc」失敗
原因：解析件數時無法取得整數
原因：invalid digit found in string
處理失敗：讀取件數文字「-1」失敗
原因：解析後的件數不合法
原因：件數 -1 不能為負數
```

兩條失敗路徑都用同一份 report。abc 尚未取得整數，−1 已取得整數卻不合法；報告保留每層工作與最終原因，沒有猜訊息文字來分類。`?` 只離開當次 helper，caller 仍可處理下一筆。

## source 的 static 不讓 reference 永久有效

`Option<&(dyn Error + 'static)>` 的外層 reference 借自這次 self；內層 bound 則要求**原因的具體型別**不依賴短命借用。只含 owned 數字的 CountError 與 ParseIntError 符合後者，reference 仍不能超過 error 的實際有效範圍。

下面 Cause 符合 static，卻由內層 Outer 擁有；Outer 結束後還使用 source 借回的 reference，仍得到 **E0597**：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/error_source_escape.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/error_source_escape.expected"
```

它不是 `&'static dyn Error`。需要在 error 結束後保留報告，可以取得前面的 owned String，而不是把原因 reference 延長。

### 外層錯誤可以有借用 context

Error 本身沒有整體 static 要求。BorrowedParseError 借一份 label，另外擁有 ParseIntError；整個錯誤依賴 label，source 借出的 cause 型別仍符合 static。

with_context 只做整數解析，不做0到100的驗證。label 的 lifetime 明寫到回傳錯誤；text 只用來取得數字，不保存在錯誤中。完整 helper 只有這份錯誤、with_context 與已教的 report：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_borrowed/errors/mod.rs"
```

完整 caller：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_borrowed/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example error_borrowed --locked
```

```text
處理失敗：資料欄位A：無法解析件數
原因：invalid digit found in string
原因借自本次error=invalid digit found in string
原標籤仍在=資料欄位A
```

report 借讀外層錯誤，沒有要求它整體符合 static。若**cause 自己**也借短命文字，就不符合標準 source 的內層 bound；以下完整負例得到 `lifetime may not live long enough`：

```rust
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/error_non_static_source.rs"
```

```text
--8<-- "examples/intro-workspace/type-stages/fixtures/compile-fail/error_non_static_source.expected"
```

BorrowedLeaf 仍能實作 Error；不成立的是 Outer 想把它當成 static 原因借出。外層 context、原因型別與這次 reference 是三個要分辨的角色。

## 報告邊界可以接收 `Box<dyn Error>`

需要以 match 穩定分類時，具體 enum 仍直接。若某個邊界只需接收不同來源並報告，可以回傳共同的 **`Box<dyn Error>`**；本頁簽名採預設 static object bound。

標準函式庫替符合 `Error + 'static` 的具體錯誤提供到這個 Box 的 From。因此第 8.6 節的 `?` 可以把 ParseIntError 或 CountError 交進 Box，保留原資料；不用為每個錯誤另寫 boxing。

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_boxed/errors/mod.rs:boxed-count"
```

這個對照省略 ReadCountError 的輸入 context，只展示兩種來源如何交給報告邊界。完整 helper 與 caller 如下：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_boxed/errors/mod.rs"
```

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_boxed/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example error_boxed --locked
```

```text
成功件數=3
處理失敗：invalid digit found in string
處理失敗：件數 101 超過批次上限100
```

caller 用 `&*error` 借 Box 內的 dyn Error，report 不接手 Box。共同 handle 沒把兩種失敗改成同一段文字，也沒有要求所有 library 介面都改用 dyn。

## 練習與完整答案

定義 OrderError，保存 id7 與 `CountError::TooLarge(101)`。derive Debug，Display 只寫 `訂單7的件數檢查失敗`，source 借出 cause。交給相同 report，確認外層訂單與原件數各出現一次。

以下是本節所有已教功能的完整 target-local helper，含四項測試；沒有載入其他節：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_complete/errors/mod.rs"
```

完整答案根檔案如下，所有設定沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/type-stages/examples/error_complete/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example error_complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-type-stages --example error_complete --locked
```

```text
處理失敗：訂單7的件數檢查失敗
原因：件數 101 超過批次上限100
```

四項測試核對具體原因、完整報告、借用 context 與 boxed 邊界。source 只借回下一層，錯誤的 owned 資料仍留在各自 owner；下一節處理資料需要多個共同 owner 的情況。

[下一節：Rc 與 Weak](11-08-lesson.md)
