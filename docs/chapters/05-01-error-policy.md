# 5.1 失敗是資料，何時才停止流程

3.2 已用 Result 保存零除數的失敗。現在我們要檢查批次件數：規則是 **0 到 100 都合法**，負數或超過上限各有不同原因。呼叫者拿到幾筆資料，要逐筆報告，不能因第一筆不合法就略過後面所有資料。

這個任務需要把失敗交給呼叫者決定；同一個下層驗證函式，也可能被只接受已保證合法值的內部流程使用。本節分開「回傳失敗資料」與「遇到某種失敗就停止正常流程」兩個選擇。

## 用具體 enum 保存原因與原輸入

我們定義 `CountError`，包含 `Negative(i32)` 與 `TooLarge(i32)`。它們的資料形狀沿用 3.1 的 enum：同時保存失敗類別與被拒絕的數字，後面的報告就不必猜原輸入。

`pub enum` 公開這個 enum 與其 variants，讓 library 外的呼叫者也能用 pattern 處理。`#[derive(Debug, PartialEq)]` 則沿用 4.2：產生檢查用的文字，以及比較相同 variant 與內容的能力，供回饋與測試使用。

`checked_count` 接收 i32，回傳 `Result<i32, CountError>`。合法時交回原值，失敗時交回具體原因；建立 Err 本身不會顯示訊息或停止程式。完整的邊界都由已教的比較判斷：

```rust
--8<-- "examples/ch05-errors/src/policy.rs:count-error"
```

本章使用 `ch05-errors` package，計算與驗證放在 library 的 `policy` module。`pub mod`、`use ch05_errors::policy::...` 與 binary 呼叫 library，仍採前文的分工。

下面的呼叫者用 match 報告每筆結果。Err arm 這次只顯示 Debug 原因，之後迴圈仍會繼續；它沒有把失敗內容當成有效數字。

```rust
--8<-- "examples/ch05-errors/src/main.rs"
```

```bash
cargo run -p ch05-errors --locked
```

```text
拒絕這筆，原因=Negative(-1)
接受件數=0
接受件數=3
接受件數=100
拒絕這筆，原因=TooLarge(101)
所有資料都已檢查
```

0 與 100 都走成功路徑。−1 和 101 保存不同失敗原因；因為呼叫者把它們當成可報告的結果，最後一句仍會執行。這就是這個任務需要 Result 的理由。

## panic 停止的是正常工作流程

**`panic!`** 是主動觸發 panic 的 macro。前文的索引越界與失敗 assertion 都曾停止正常流程，現在可以明確寫 `panic!("原因={value}")`；格式字串的插入方式和 println 相同。

在本節的 main 範例中發生 panic 時，正常流程不會再往下，程式會顯示失敗訊息與位置並結束。這可以表達內部程式契約被破壞，讓錯誤在第一次遇到的位置顯現；若正常外部資料本來就可能不合法，停止所有工作就不符合剛才逐筆報告的需求。

下面故意宣告一項不同的內部契約：take_positive 的輸入必須是正數。呼叫端卻傳 0，所以 fixture 會 panic，而不是回傳一個成功數字：

```rust
--8<-- "examples/ch05-errors/fixtures/runtime-fail/panic_contract.rs"
```

把它放進練習專案執行，可看到 `內部契約要求正數`。這份 fixture 用來觀察停止行為；它不是對所有可能有零的件數資料都採用的政策。

## unwrap 與 expect 也會替失敗選擇 panic

以下都是標準 enum 的新 method，先讀各自的行為。它們接收整個結果值，交出裡面的成功值；對不是 Copy 的結果，呼叫會消耗該結果，不能再用原名稱。

| method | 成功／有值時 | 失敗／缺值時 |
| --- | --- | --- |
| Result 的 `unwrap()` | 交出 Ok 中的值 | panic，附上 Err 的 Debug 內容 |
| Result 的 `expect(message)` | 交出 Ok 中的值 | panic，附上自己提供的訊息與 Err 的 Debug 內容 |
| Option 的 `unwrap()` | 交出 Some 中的值 | None 時 panic |
| Option 的 `expect(message)` | 交出 Some 中的值 | None 時 panic，附上自己提供的訊息 |

Result 的兩種 method 都要求錯誤型別有 **Debug**，因為遇到 Err 時要印出原因。`CountError` 已提供這個能力；Option 的 None 則沒有一個錯誤值要顯示。

下面只對程式固定建立的合法件數，以及已確認位置的 slice get 結果 使用這些 method。`expect` 的訊息說明哪項保證使成功成為合理預期；它沒有讓原本可能失敗的工作變成不會失敗。

```rust
--8<-- "examples/ch05-errors/examples/unwrapping.rs"
```

```bash
cargo run -p ch05-errors --example unwrapping --locked
```

輸出是固定件數 3、空批次件數 0，以及兩種 method 從相同索引取得的 4、4。若資料改為外部提供，就要重新評估是否仍有這項保證，而不是照抄固定範例的 unwrap。

### 確認失敗分支確實會停止

下面先借用一個必然失敗的結果，顯示被拒絕的件數，再把它交給 unwrap。fixture 中只保留超上限這一種錯誤，用來聚焦 unwrap 的行為：

```rust
--8<-- "examples/ch05-errors/fixtures/runtime-fail/unwrap_error.rs"
```

執行會看到 `TooLarge(101)` 的 panic 內容，不會取得一個合法件數。對 None 使用 expect 的 [另一個完整 fixture](https://github.com/birdhackor/learn_rust/blob/main/examples/ch05-errors/fixtures/runtime-fail/expect_error.rs) 也會停止，訊息是 `本案例故意沒有預期中的數字`。

若錯誤型別缺少 Debug，就連 unwrap 呼叫本身也不符合契約。這個 [compile-fail fixture](https://github.com/birdhackor/learn_rust/blob/main/examples/ch05-errors/fixtures/compile-fail/unwrap_without_debug.rs) 會以 `E0277` 指出缺少能力；這和 4.1–4.2 檢查 trait 要求是同樣的關係。

## 用測試區分正常邊界與失敗原因

標準 Result 在成功型別與錯誤型別都有 PartialEq 時，也能比較：先分辨 Ok／Err，再比較同一種 variant 的內容。兩側皆可 Debug 顯示時，就能用 assert_eq 在失敗時列出差異；這延伸 4.2 的 enum 與 4.3 的 Option 比較契約。

下列測試保存 0、100 的合法邊界，再檢查 −1、101 的失敗分類和原輸入：

```rust
--8<-- "examples/ch05-errors/src/policy.rs:policy-tests"
```

```bash
cargo test -p ch05-errors --locked
```

測試需要有保證的範例值才能決定預期結果。測試內的 assertion 失敗會使該項測試失敗；這個目的與處理外部資料時繼續報告是不同工作。

## 練習：讓呼叫者提供清楚的拒絕訊息

寫 show_input，接收 i32，再呼叫 checked_count。合法就顯示件數；Negative 就顯示「不能為負數」與原值；TooLarge 就顯示「超過批次上限100」與原值。讓 −1、0、100、101 依序都被處理，保留原本每筆獨立報告的政策。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch05-errors/examples/policy_solution.rs)，或執行 `cargo run -p ch05-errors --example policy_solution --locked`。這裡用 pattern 決定報告；稍後會把型別本身的顯示契約也定義出來。

下一節先給重複出現的 Result 型別一個較短名字，讓後面的格式化接口容易閱讀。
