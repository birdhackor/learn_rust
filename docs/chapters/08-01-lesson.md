# 8.1 Result 與 panic：回報失敗，或停止正常流程

第 5.2 節已用 Result 保存除數零的失敗。現在要檢查批次件數：**0 到 100 都合法**，負數與超過上限各有不同原因。呼叫者收到幾筆資料，就要逐筆報告，不能因第一筆不合法而略過其餘資料。

同一個驗證函式也可能用在另一種情況：內部流程已保證輸入合法，若這項保證被破壞，就停止正常工作。本節先完成逐筆報告，再看明確選擇停止的方式。

## 用 enum 保存失敗原因與原輸入

`CountError` 包含 `Negative(i32)` 與 `TooLarge(i32)`，分別保存負數、超過上限的原因，以及被拒絕的原數字。這沿用第 4.1 節帶資料的 enum；`pub enum` 讓其他 module 也能用其 variant 做 pattern 配對。

`#[derive(Debug, PartialEq)]` 沿用第 6.2 節：產生檢查用的文字，以及比較 variant 與內容的能力。兩個 variant 中的 i32 都具備這些能力。

`checked_count` 接收 i32，交回 `Result<i32, CountError>`：合法時把原值放進 Ok，否則把原因放進 Err。建立 Err 本身不會顯示訊息、觸發 panic 或停止程式：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/policy/policy/mod.rs:count-error"
```

本章使用 `examples/intro-workspace/error-stages/`，package 名稱為 `intro-error-stages`。它沒有 library；每個 example 都只載入當時已教的完整模組。這個 target 的根檔案是 `examples/policy/main.rs`，`pub mod policy;` 從同層的 `policy/mod.rs` 載入上面定義，沿用第 2.6 節的規則。`crate::` 從這個 example 的根開始。

## 呼叫者決定：報告後繼續下一筆

下面的 main 用第 4.2 節的 match 分辨 Ok 與 Err。Err arm 只顯示原因的 Debug 文字，迴圈接著處理下一筆；沒有把被拒絕的數字當成合法件數：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/policy/main.rs"
```

從教材 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example policy --locked
```

```text
拒絕這筆，原因=Negative(-1)
接受件數=0
接受件數=3
接受件數=100
拒絕這筆，原因=TooLarge(101)
所有資料都已檢查
```

第一筆 −1 被拒絕，後面的 0、3、100 仍會被接受。最後一句也有執行，表示呼叫者完成了整批檢查。Result 讓驗證函式交出成功值或失敗原因，呼叫者才能依「逐筆報告」的需要決定後續。

## panic!：明確停止正常流程

**`panic!`** 是主動觸發 panic 的 macro。前面的索引越界或 assertion 失敗也曾停止正常流程；現在可以直接寫 `panic!("原因={value}")`，格式字串插入值的方式與 println 相同。

在本頁這種由單一 main 執行的程式中，發生 panic 後不再繼續正常往下執行，會顯示訊息及程式位置，並以失敗狀態結束。這能把被破壞的內部保證顯現在發生處；但正常輸入本來就可能不合法時，逐筆報告需要的是前面的 Result 政策。

下面是完整的**預期執行失敗**程式。它另外規定 `take_positive` 只接受正數，故意傳入 0 來觀察 panic；這不是把前面的批次規則改成不接受零：

```rust
--8<-- "examples/intro-workspace/error-stages/fixtures/runtime-fail/panic_contract.rs"
```

若要單獨觀察，把完整程式放入第 1.2 節獨立練習專案的 `src/main.rs`，執行 `cargo run`。先看到 `開始檢查內部契約`，接著在失敗訊息中看到 `內部契約要求正數，實際值=0`；不會看到最後的「檢查後」。它沒有回傳一個成功件數讓 main 繼續。

## unwrap 與 expect：取出值，並替失敗選擇 panic

**`unwrap()`** 與 **`expect(message)`** 是 Result、Option 提供的方法。它們以 `self` 接收整份結果，成功時交出裡面的值；若該結果型別不是 Copy，呼叫會消耗它，不能再用原名稱。`message` 是你提供的 `&str` 說明。

標準 `Result<T, E>` 只有在 T、E **兩者都是 Copy** 時才有 Copy。本頁的 CountError 沒有 Copy，因此 `Result<i32, CountError>` 會 move 進這些方法；裡面的成功數字是 Copy，不代表整份結果也能 Copy。

| 方法 | 成功／有值時 | 失敗／缺值時 |
| --- | --- | --- |
| Result 的 `unwrap()` | 交出 Ok 中的值 | panic，包含 Err 的 Debug 內容 |
| Result 的 `expect(message)` | 交出 Ok 中的值 | panic，包含 message 與 Err 的 Debug 內容 |
| Option 的 `unwrap()` | 交出 Some 中的值 | None 時 panic |
| Option 的 `expect(message)` | 交出 Some 中的值 | None 時 panic，包含 message |

Result 的兩個方法都要求錯誤型別有 **Debug**，即使這次預期只會取得 Ok，也要符合這項型別契約。CountError 已提供 Debug；Option 的 None 則沒有錯誤值需要顯示。

下面固定輸入 3 與 0，依本頁規則一定合法。`expect` 訊息記下這項預期的原因；它沒有替任何輸入補做保證：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/unwrapping/main.rs:result-values"
```

Option 例子則使用第 5.4 節的 slice `get`：它借讀 slice，交回 `Option<&i32>`。我們已建立兩筆資料，所以索引 1 存在，Some 內是借到第二筆數字的 reference。unwrap 與 expect 取出的是這份 reference；最後用第 3.3 節的 `*` 讀到數字：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/unwrapping/main.rs:option-values"
```

完整根檔案如下；它的 policy 模組完整快照列在本頁後面：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/unwrapping/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example unwrapping --locked
```

```text
固定件數=3，空批次件數=0
Some中的數字=4、4
```

這些是能直接指出成功保證的固定材料。若改成外部提供的件數或索引，就要重新評估保證；對本來可能失敗的正常資料，應像開頭一樣明確處理結果，而不是為取得數字而強行 unwrap。

### 遇到 Err 或 None，確實會停止

第一份完整 fixture 只保留超上限這一種錯誤。它先以第 4.5 節的 borrowed pattern 讀取原因：`match &result` 借讀結果，value 是 `&i32`，因此用 `*value` 取數字；借讀結束後才把整份 result 交給 unwrap。CountError 沒有 Copy，這一步會接手結果。

```rust
--8<-- "examples/intro-workspace/error-stages/fixtures/runtime-fail/unwrap_error.rs"
```

放入獨立練習專案執行，先顯示 `被拒絕的件數=101`，接著 panic 訊息包含 `TooLarge(101)`。沒有取得一個合法件數。

下一份對 None 使用 expect，也是一份完整的預期執行失敗程式：

```rust
--8<-- "examples/intro-workspace/error-stages/fixtures/runtime-fail/expect_error.rs"
```

panic 訊息包含 `本案例故意沒有預期中的數字`。這個訊息刻意標明失敗示範；實際用於有保證的流程時，應像前面的固定批次、索引例子，寫出**為什麼預期有值**。

若缺少 Debug，Result 的 unwrap 呼叫連編譯契約也不符合。下面是完整的**預期編譯失敗**程式：

```rust
--8<-- "examples/intro-workspace/error-stages/fixtures/compile-fail/unwrap_without_debug.rs"
```

放入獨立練習專案執行 `cargo check`，診斷 **E0277** 指出 CountError 缺少 Debug。這和第 6.1、6.2 節使用某項能力前必須提供相應 trait 的關係相同。

## 用測試保留邊界與不同失敗原因

第 6.2 節已比較整份 Option。標準 **`Result<T, E>`** 在 T 與 E **兩者都有 PartialEq** 時，也能比較：Ok 與 Err 不相等；同為 Ok 比成功值，同為 Err 比錯誤值。T 與 E 兩者都有 Debug 時，整份 Result 也能 Debug 顯示。

所以這裡的 `Result<i32, CountError>` 能使用 `assert_eq!`：PartialEq 判斷結果相等，Debug 供 assertion 失敗時顯示差異。比較借讀左右兩邊，沿用第 6.2 節的 assertion 契約。

下面是 policy 範例實際載入的**完整** `policy/mod.rs`，包含前面的定義與兩項測試。第一項確認 0、100 都合法；第二項確認 −1、101 分別保留原因與原值：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/policy/policy/mod.rs"
```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example policy --locked
```

測試內 assertion 失敗時，該項測試應失敗；這與正常資料驗證後仍要繼續報告，是不同的工作目的。

本節所有成功程式使用的完整 member manifest 只有以下三個已教欄位，沒有額外依賴或 library：

```toml
--8<-- "examples/intro-workspace/error-stages/Cargo.toml"
```

??? note "unwrapping target 的完整 policy 快照"

    這是該 target 同層的 `policy/mod.rs`，內容與前面的驗證定義、測試相同；沒有載入格式化或轉換模組。

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/unwrapping/policy/mod.rs"
    ```

## 練習與完整答案

寫 `show_input` 接收 i32，呼叫 checked_count：合法時顯示件數；Negative 時顯示「不能為負數」與原值；TooLarge 時顯示「超過批次上限100」與原值。讓 −1、0、100、101 都依序被處理，最後顯示四筆都已回報。

用巢狀 pattern 分開兩種錯誤，保留每筆獨立報告的政策。下面是完整答案的根檔案，兩項 assertion 也核對合法邊界：

```rust
--8<-- "examples/intro-workspace/error-stages/examples/policy_answer/main.rs"
```

??? note "答案實際載入的完整 policy/mod.rs"

    ```rust
    --8<-- "examples/intro-workspace/error-stages/examples/policy_answer/policy/mod.rs"
    ```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-error-stages --example policy_answer --locked
```

```text
件數 -1 不能為負數
接受件數=0
接受件數=100
件數 101 超過批次上限100
四筆資料都已回報
```

這裡仍由呼叫者選報告方式。下一節先替重複出現的 Result 型別取較短名稱，再讓型別本身提供顯示契約。

[下一節：type alias](08-02-lesson.md)
