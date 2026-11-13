# 8.4 From、Into 與 TryFrom：轉換與驗證件數

checked_count 雖然檢查 0 到 100，成功後卻仍交回普通 i32。另一個函式只看見 i32，無法從型別辨認它是否已通過檢查。這次建立 Count：只有符合規則的數字才能由公開操作建立這個型別，接收 Count 的工作便能使用已驗證件數。

先認識標準轉換介面，再用它安排「普通整數→有效件數→取回整數」的往返。

## From 與 Into 表達保證成功的轉換

**`From`** 是標準的轉換 trait，使用型別參數指定來源。`From<i32>` 表示從 i32 轉過來；`impl From<i32> for i64` 的來源是 i32，目的型別與 Self 是 i64。trait 使用型別參數，沿用第 5.1、6.1 節的泛型與實作語法。

**`From::from`** 以值接收來源，直接交回目的型別，不回傳 Result。它的介面約定是一定成功、保留有意義的原資訊，而且轉換意義明確；可能失敗的驗證不能用 panic 假裝成這種轉換。

**`i64`** 是 64-bit 有號整數，能表示所有 i32 的整數。標準函式庫已提供 i32→i64 的 From，轉換後仍是同一個數字，因此可呼叫 `i64::from(count)`。i64 也有標準數字的 Debug 與 PartialEq，下面能用 assertion 核對結果。

**`Into`** 從來源這一側表達轉換：`Into<i64>` 的型別參數是目的型別，`count.into()` 交出目的值。當目的型別有 `From<來源>`，標準函式庫就提供來源的 `Into<目的>`，不必再寫一份相同邏輯。這是由 From 得到 Into，不能反過來只因看見某份 Into 就假定一定有 From。

`.into()` 的方法名本身沒有指明目標，下面用 `let by_into: i64` 提供型別線索。兩種方式都以值接收來源；i32 是 Copy，所以原 count 仍可讀。

From、Into 與稍後的 TryFrom 都在本專案 edition 2024 的 **prelude** 中，沿用第 5.2 節預先匯入名稱的角色。不需為這些標準 trait 加依賴；範例仍可明寫 `use std::convert::...;`，方便追蹤名稱來源。

```rust
--8<-- "examples/intro-workspace/error-stages/examples/from_into.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example from_into --locked
```

```text
From：3
Into：3
原本的 i32：3
```

## 私有欄位讓建立入口負責驗證

Count 的 value 仍是 i32。第 3.1 節分開型別與欄位的 pub；這次只公開 struct，**欄位沒有 pub**，因此 conversions module 外的呼叫端不能直接寫 `Count { value: -1 }`。公開的 `value(&self)` 只借讀並交回數字，沒有開放修改欄位。

Count 產生 Debug、PartialEq，供檢查與測試；沒有產生 Copy，所以之後以值轉換它會 move。下面先定義型別與借讀方法，接著才提供經過檢查的建立入口：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/conversions/conversions/mod.rs:count-model"
```

## TryFrom 保留轉換失敗與錯誤型別

普通 i32 可能是 −1 或 101，轉成 Count 不能保證成功。**`TryFrom<i32>`** 因此要求 `try_from` 以值接收 i32，交回 `Result<Self, Self::Error>`：成功是目的型別，失敗則是這份實作選定的錯誤型別。

這裡回收第 6.4 節的 associated type。TryFrom 要求的型別角色叫 Error；`type Error = CountError;` 選定它，所以本份實作的 Self 是 Count、Self::Error 是 CountError，整個結果實際是 `Result<Count, CountError>`。

方法先呼叫既有 checked_count。只有 Ok 分支才建立 `Self { value }`，沿用第 3.6 節同名欄位簡寫；Err 分支保留原原因，不把被拒絕的數字包成 Count：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/conversions/conversions/mod.rs:count-try-from"
```

此處的 `type Error` 位於 trait impl，是填入契約要求的 associated type；不是在 module 另取一個可以直接寫 Error 的別名。

## 從 Count 取回 i32：借讀或接手

Count 已保存合法 i32，反方向取回整數不需再驗證，所以適合 `From<Count> for i32`。這份實作的 Self 是 i32，來源參數則是整個 Count：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/conversions/conversions/mod.rs:count-from"
```

`i32::from(count)` 與目標為 i32 的 `count.into()` 都接手 Count。若只想看數字並保留它，就呼叫 `count.value()` 借讀。兩種操作取得相同數字，對 ownership 的作用卻不同。

完整呼叫者如下。其 target 根只載入本頁完整提供的 conversions、policy，以及第 8.3 節的 formatting；後者讓 Err arm 能直接顯示 CountError 的 Display 訊息：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/conversions/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example conversions --locked
```

```text
已驗證件數：0
取回 i32：0
已驗證件數：3
取回 i32：3
已驗證件數：100
取回 i32：100
轉換失敗：件數 -1 不能為負數
轉換失敗：件數 101 超過批次上限100
```

0、3、100 先得到 Count，再取回同一個數字。−1 與 101 根本沒有得到 Count；呼叫者回報失敗後仍繼續。這項有效性保證來自公開介面：建立時檢查，公開方法不提供破壞規則的修改。同一個 conversions module 仍可存取私有欄位，實作者新增操作時也必須維持規則。

## 完整來源與往返測試

完整 member manifest 與前幾節相同，沒有 library 或額外依賴：

```toml
--8<-- "examples/intro-workspace/error-stages/Cargo.toml"
```

下面是 conversions target 的完整 conversions/mod.rs。三項測試核對有效邊界、相鄰無效值與取回原數字。Result 比較依第 8.1 節：成功的 Count、失敗的 CountError 都有 PartialEq／Debug。固定合法測試資料的 expect 訊息也指出成功保證：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/conversions/conversions/mod.rs"
```

它還載入以下兩份完整 helper；policy 兩項、formatting 一項，加上轉換三項，這個 target 共六項測試：

??? note "conversions target 的完整 policy/mod.rs"

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/conversions/policy/mod.rs"
    ```

??? note "conversions target 的完整 formatting/mod.rs"

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/conversions/formatting/mod.rs"
    ```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example conversions --locked
```

## 用 never type 表達不可能產生的錯誤

前面的 i32→Count 需要驗證，所以 Error 是 CountError；i32→i64 則能保留所有來源整數，不需要失敗結果。直接做後一種轉換時，使用 i64::from 就足夠。

如果某段共同流程使用 TryFrom 介面，標準函式庫也為已提供 From 的轉換提供 TryFrom；此時仍回傳 Result，但錯誤型別是 **`std::convert::Infallible`**。從 Rust 1.100 起，這個名稱是第 8.1 節 **`!`** 的 type alias，沒有任何可建立的值。因此 `Result<i64, !>` 可以是 Ok，卻不可能建立 `Err(錯誤值)`。

以下 convert 以 `T: TryFrom<i32>` 要求目的型別提供轉換，交回 `Result<T, T::Error>`；`T::Error` 是該實作選定的 associated type。這沿用第 5.1 節泛型函式與第 6.4 節的輸出關係，讓同一個 helper 可以保留不同目的型別的錯誤契約。本次指定目的為 i64，觀察錯誤分支不可能成立的情況，沒有為保證成功的工作新增一次驗證：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/infallible_conversion.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example infallible_conversion --locked
```

```text
沒有Err分支的轉換=3
Infallible別名的轉換=-1
```

第 4.3 節的普通 let pattern 必須涵蓋所有可能的形態。這裡 `let Ok(value)` 能通過，是因為 Err 的內容不可能存在；不是讓 let 忽略失敗。第二筆能指定 `Result<i64, Infallible>`，則確認別名與 `!` 是同一個型別。這項保證只說不會回傳 Err，不保證任意使用這個簽名的函式不會 panic。

延伸閱讀：[never type](https://doc.rust-lang.org/1.100.0/std/primitive.never.html)、[Infallible](https://doc.rust-lang.org/1.100.0/std/convert/type.Infallible.html)。

## 練習與完整答案

寫 `count_value(value: i32) -> Result<i32, CountError>`：先以 Count::try_from 驗證，再用 i32::from 取回數字。不要重寫0到100的條件，失敗時保留原 CountError。確認0、100、−1、101四個輸入。

完整根檔案仍用 match 處理完整結果，沒有縮寫或額外API：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/conversion_answer/main.rs"
```

??? note "答案實際載入的全部三份完整 helper"

    `conversions/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/conversion_answer/conversions/mod.rs"
    ```

    `policy/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/conversion_answer/policy/mod.rs"
    ```

    `formatting/mod.rs`：

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/conversion_answer/formatting/mod.rs"
    ```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example conversion_answer --locked
```

```text
轉換往返保留有效件數，也保留失敗原因
```

驗證規則仍在 checked_count 一處，Count 的建立入口與取回操作負責各自的轉換。下一節把來源改成文字，再分清解析、借出視圖與建立 owned 結果。

[下一節：FromStr、AsRef 與 owned 轉換](08-05-lesson.md)
