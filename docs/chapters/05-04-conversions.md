# 5.4　From、Into 與 TryFrom：轉換是否可能失敗？

上一節把 `CountError` 顯示成讀得懂的訊息，這一節回到件數本身。`checked_count` 雖然會檢查 0 到 100，但成功後仍回傳普通的 `i32`；另一個函式只看到 `i32`，無法從型別辨認它是否已經通過檢查。

我們先學標準的型別轉換介面，再建立 `Count`：只有符合規則的數字才能轉成這個型別。它讓接收 `Count` 的函式知道檢查已經完成，而不必重新檢查每個普通整數。

## 保證成功的轉換：From 與 Into

`From` 是標準庫的轉換 trait。它使用型別參數指定來源：`From<i32>` 表示「從 `i32` 轉換過來」。在 `impl From<i32> for i64` 中，來源是 `i32`，目的型別及 `Self` 是 `i64`。這是第 3 章泛型語法用在 trait 上；第 4 章的 trait 方法契約仍然適用。

`From::from` 接收一個來源值，直接回傳目的型別的值，不回傳 `Result`。這個介面的約定是轉換一定成功、保留原本有意義的資訊，而且轉換意義明確；不能把可能失敗的檢查藏成 `panic!`。

標準庫已提供 `i32` 到 `i64` 的 `From` 實作：`i64` 能表示所有 `i32` 的整數，轉換後仍是同一個數字。因此可以呼叫 `i64::from(count)`。

`Into` 從另一個方向表達同一件事：`Into<i64>` 指定目的型別，`count.into()` 把接收者轉成它。當某個目的型別實作 `From<來源型別>`，標準庫就會提供對應的 `Into<目的型別>`，不必再寫第二份轉換邏輯。`.into()` 有時無法單靠方法名稱決定目的型別，下面用 `let by_into: i64` 指定它。

兩種方法都以值接收來源，適用第 2 章的 Copy／move 規則。這裡的 `i32` 具有 `Copy`，所以轉換後仍可讀取 `count`。

本專案的 edition 2024 會透過 **prelude（預先匯入的常用名稱）** 提供 From、Into 與稍後的 TryFrom。呼叫端因而可以直接使用它們的轉換方法。有些範例仍明寫 `use std::convert::...;`，讓標準庫的來源路徑可追蹤；自訂 trait 則繼續沿用 4.1 在呼叫端用 use 引入的安排。

```rust
--8<-- "examples/ch05-errors/examples/from_into.rs"
```

```bash
cargo run -p ch05-errors --locked --example from_into
```

```text
From：3
Into：3
原本的 i32：3
```

## 建立只有有效件數才能取得的型別

接下來定義 `Count`，裡面的 `value` 仍是 `i32`。struct 是 `pub`，field 沒有 `pub`，所以 library 外的呼叫者能使用 `Count`，卻不能直接寫 `Count { value: -1 }`。公開的 `value(&self)` 方法只借用它、讀出數字，不修改 field。

這次為 `Count` 加上已學過的 `Debug` 與 `PartialEq`，讓測試能顯示與比較轉換結果；沒有讓它實作 `Copy`。之後轉換時，仍要注意它的 ownership。

```rust
--8<-- "examples/ch05-errors/src/conversions.rs:count-model"
```

## 可能失敗的轉換：TryFrom 與錯誤型別

普通 `i32` 可能是 -1 或 101，不能保證轉成 `Count`。因此使用 `TryFrom<i32>`。它的 `try_from` 方法接收來源值，回傳 `Result<Self, Self::Error>`：成功是目的型別，失敗是這個實作指定的錯誤型別。

這裡出現 **associated type（關聯型別）**。除了要求方法，trait 也可以要求每個實作指定某個角色的型別。`TryFrom` 要求的角色叫 `Error`；我們在實作內寫 `type Error = CountError;`，表示這個轉換失敗時使用既有的 `CountError`。方法簽名中的 `Self::Error` 就是這個實作選定的型別。

它與第 5.2 節獨立的 type alias 位置不同：`CountResult` 是可以直接使用的型別名稱；這裡的 `Error` 是 `TryFrom` 要求實作填入的一個型別位置。此刻先掌握「這個實作選哪種錯誤」即可。

下面 `Self` 是 `Count`，所以回傳型別實際上是 `Result<Count, CountError>`。方法沿用 `checked_count` 的規則：只有 `Ok(value)` 才建立 `Count`，`Err(error)` 則保留原本的失敗原因。`From` 與 `TryFrom` 由 `std::convert` 提供；這個 module 以逐條 `use` 引入它們。

```rust
--8<-- "examples/ch05-errors/src/conversions.rs:count-try-from"
```

## 從有效件數取回整數

反方向從 `Count` 取回 `i32`，一定能保留裡面的數字，因此適合 `From`。下面的 `Self` 是 `i32`，參數則是整個 `Count` 值。

```rust
--8<-- "examples/ch05-errors/src/conversions.rs:count-from"
```

呼叫 `i32::from(count)` 或以目的型別為 `i32` 的 `count.into()`，都會 move 這個 `Count`，因為它沒有 `Copy`。若只想查看數字、繼續持有 `Count`，使用前面借用它的 `count.value()`。這兩種操作回傳相同數字，但對 ownership 的作用不同。

```rust
--8<-- "examples/ch05-errors/examples/conversions.rs"
```

```bash
cargo run -p ch05-errors --locked --example conversions
```

輸入 0、3、100 會先列出已驗證件數，再列出取回的同一個整數；-1 與 101 會列出第 5.3 節建立的失敗訊息。檢查失敗時根本沒有取得 `Count`，不需要在後續操作中把無效件數當成成功值處理。

這項保證依賴我們公開的介面：建立 `Count` 的轉換有檢查，公開方法也沒有提供修改 field 的路徑。同一個 module 仍能接觸私有 field，所以實作者新增方法時也要維持這項規則。

## 驗證界線與轉換往返

測試要確認有效邊界能建立 `Count`、相鄰無效值保留原因，以及轉回 `i32` 不改變件數。`Result` 能比較是因為成功的 `Count` 與失敗的 `CountError` 都已實作 `PartialEq`；這是第 5.1 節用過的比較條件。

```rust
--8<-- "examples/ch05-errors/src/conversions.rs:conversion-tests"
```

```bash
cargo test -p ch05-errors --locked
```

練習：寫 `fn count_value(value: i32) -> Result<i32, CountError>`，先以 `Count::try_from` 檢查，再以 `i32::from` 取回整數。不得重寫 0 到 100 的條件，失敗時保留原本的 `CountError`。確認 0、100、-1、101 四個輸入。

??? note "參考答案"

    ```rust
    --8<-- "examples/ch05-errors/examples/conversion_solution.rs"
    ```

    執行：`cargo run -p ch05-errors --locked --example conversion_solution`。
