# 5.2 Option 與 Result：分清有值、缺值、成功與失敗

第 4 章用自訂 Quote 區分缺少報價與已有金額。許多工作只需要「有一個值，或沒有值」這種共同形狀；另一類工作還需要保存失敗原因。標準函式庫分別提供 Option 與 Result，現在用上一節的型別參數讀懂它們。

## Option：有值與缺值

**`Option<T>`** 是標準的 enum，只有一個型別參數 T：**Some(T)** 保存一份 T 型別的值，**None** 沒有附帶值，表示缺值。因此 `Option<i32>` 是「可能有一個 i32」，本身不是 i32；`Option<bool>` 則可能有一個 bool。

`Some(120)` 是有報價 120，`Some(0)` 是有報價零，None 才是沒有報價。不能直接把整份 `Option<i32>` 當金額乘數量，必須先處理它屬於哪一種情況。

這些標準名稱不需下載或自行定義。Rust 的 **prelude**（預先匯入的常用名稱）會自動把一組名稱帶進一般 module；本頁的 Option、Result 與它們的 variant 都在其中。因此可直接寫 Some、None，稍後也能直接寫 Ok、Err，不必像自訂 enum 一樣每次加型別前綴。它們仍然是 variant，不是數字或控制流程關鍵字。

建立時把具體型別填入角括號：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/option_result.rs:options"
```

None 沒有內容可提供 T 的線索，所以第一行直接註記 `Option<i32>`。Some 的內容能提供推導線索；後兩行仍明寫型別，方便對照 i32 與 bool。這不表示每個 None 都必須自己加註記：只要同一份結果的其他使用位置已提供足夠線索，也能推導。

## 用 pattern 取得內容，再決定工作

接受 `Option<i32>` 的函式可以用第 4 章的 match：Some(amount) 取得裡面的 i32，None 分支處理缺值。這裡只顯示，不把缺值擅自改成價格零。

```rust
--8<-- "examples/intro-workspace/data-steps/examples/option_result.rs:show-option"
```

函式以值接收這份 Option，main 呼叫後不再使用傳出的那個名稱。完整程式也對 `Option<bool>` 使用 `if let Some(ready)`，只在有設定時顯示 bool，沿用第 4.6 節「只關心一種形態」的規則。

接著用 bool 決定這次是否提供零元報價。if 的兩個分支都產生同一型別 `Option<i32>`；Some(0) 中的整數提供推導線索：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/option_result.rs:choose-option"
```

available 為 true 時顯示 `取得報價=0`；為 false 時顯示 `缺少報價`。兩種結果因此不會混在一起。

## Result：成功值與失敗原因各有型別

現在要計算 `12 / 2`，也要面對除數零。**`Result<T, E>`** 是另一個標準 enum，使用**兩個型別參數**：

| 參數與 variant | 保存的角色 | 本例的具體型別 |
| --- | --- | --- |
| T，`Ok(T)` | 成功取得的值 | i32 整數商 |
| E，`Err(E)` | 失敗資訊 | 自訂 DivideError |

T 和 E 是型別位置，不是兩個執行時的數字。`Result<i32, DivideError>` 表示成功時有 i32、失敗時有 DivideError。DivideError 的 ZeroDenominator variant 表示除數零，不需要額外數字欄位。

因此 `divide(numerator: i32, denominator: i32) -> Result<i32, DivideError>` 能在簽名就說清完整結果。函式先判斷除數零，該分支只建立 Err；非零分支才執行除法，將整數商放進 Ok：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/option_result.rs:divide"
```

Err 保存的是資料，建立它本身不會顯示訊息、panic 或停止整個程式。呼叫者用 match 決定如何回應。這裡的失敗 pattern 由外往內，先配對 Err，再配對它帶著的 ZeroDenominator：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/option_result.rs:handle-result"
```

除數 2 得到 Ok(6)，value 是 i32；除數 0 得到 Err，顯示原因後仍可繼續後面的工作。本例只處理不會溢位的小整數與零除數，沒有宣稱涵蓋所有 i32 邊界運算。

## 完整程式與執行

本頁使用沒有 library 的 `intro-data-steps`，完整設定與全部 helper、main 如下：

```toml
--8<-- "examples/intro-workspace/data-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/data-steps/examples/option_result.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example option_result --locked
```

```text
缺少報價
取得報價=120
設定值=true
取得報價=0
缺少報價
整數商=6
除數必須非零
```

這些輸出來自 pattern 取出的數字／bool 或選定分支的固定文字，沒有直接顯示整個 Option 或 Result。

## 按呼叫者需要保留資訊

| 工作結果需要回答什麼 | 適合的形狀 | 接收者決定的事 |
| --- | --- | --- |
| 有沒有取得報價，不需另外分類缺少原因 | `Option<T>` | 有值如何使用、缺值如何處理 |
| 是否成功，失敗時還要保留原因 | `Result<T, E>` | 成功值如何使用、錯誤原因如何處理 |

None 不會告訴你為什麼缺值；Err 則能保存我們選定的錯誤型別。兩者都要求接收者先分辨情況，不會自動替工作選出假數字。

## 練習與完整答案

定義 AmountError::Negative，寫 `checked_amount(amount: i32) -> Result<i32, AmountError>`：金額非負時交出 Ok 中的原金額，負數時交出 Err。讓 main 依序處理 0、−5，預期顯示接受零元，再拒絕負數。

簽名中的成功型別是 i32，錯誤型別是 AmountError；兩個位置都先確定。完整答案的 assertion 只檢查這兩筆已選材料取出的 i32／bool，不比較整份 Result：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/result_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example result_answer --locked
```

```text
接受金額=0
拒絕負數金額
```

函式成功取得零，和函式沒有成功值，是兩種不同結果。下一節會把 Option 用在文字範圍：合法空文字也有值，無效區段則沒有取得文字。

[下一節：char、UTF-8 與文字範圍](05-03-lesson.md)
