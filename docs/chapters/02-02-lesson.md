# 2.2 用 if、else 與 return 選擇工作

現在已能用 `item_total >= 300` 判斷是否達到免運門檻。接下來要真的選出運費：商品小計至少 300 元收 0 元，低於 300 元收 20 元；數量為零時，整筆訂單總額為零。

本節的訂單單價與數量都採用非負的小整數，乘法與加法不會超出 `i32` 範圍。先把這項計價規則寫清楚，外部輸入的有效性後面再處理。

## if 先檢查，只執行被選到的 block

`if 條件 { 工作 } else { 另一項工作 }` 先算條件。得到 true 時只執行第一個 block，得到 false 時只執行 `else` 的 block。兩邊的程式不會都做一次。

第一章已教過 block 能交出尾端 expression 的值。`if` 也是 expression，因此這裡可以讓兩個分支分別交出 0 或 20，成為運費：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/conditions.rs:shipping-value"
```

小計 160 時選出 20；恰好 300 時選出 0。兩邊都是 `i32`，所以 `shipping` 能保存一個明確型別的數字。若要把 `if` 的結果當成數字使用，每個會交出結果的分支都要提供相容的數字型別；不能一邊交出 `i32`，另一邊只有 `println!` 的 unit 結果。

最後的 `;` 結束 `let shipping = ...;`。分支中的 `0`、`20` 沒有分號，才能留下選中的數值；這與第一章的 block 規則相同。

條件必須是 `bool`。下面是完整的**預期編譯失敗**程式：

```rust
--8<-- "examples/intro-workspace/control-steps/fixtures/compile-fail/if_number.rs"
```

錯誤重點是 `expected bool, found i32`。Rust 不會像 Python 那樣把非零整數自動視為成立。若要表達數量為正，條件應寫成 `count > 0`。

## return 提早結束目前函式

數量為零時，答案已經是零，不必再計算商品小計或運費。

`return 0;` 立刻結束**這次函式呼叫**，把 0 交回呼叫者；函式後面的工作不再執行。它和函式尾端 expression 都能交出結果，差別是 `return` 能在中途離開。

把這一段放在計價函式開頭：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/conditions.rs:early-return"
```

這裡沒有 `else`：條件不成立時，直接接著做 if 後面的工作。因為成立時已經 return，兩種情況仍能分開。`return` 結束的是函式，並非只離開這個 if block。

## 先檢查索引，再決定要不要讀取

第一章的索引 3 曾讓三項 array 越界。現在可以在讀取之前選擇路徑：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/conditions.rs:index-check"
```

`show_selected_price` 與第一章的索引函式一樣，從呼叫者收到 `usize` 參數。完整程式最後呼叫 `show_selected_price(3)`；這次 `3 < 3` 是 false，因此只顯示索引無效，沒有執行 `prices[index]`。改成傳入索引 1 時，則只顯示選到價格 80。這裡的 if 用來選工作；兩邊都是顯示，不需要把整個 if 保存成金額。

## 完整範例與執行

本節使用的完整設定與上一節相同：

```toml
--8<-- "examples/intro-workspace/control-steps/Cargo.toml"
```

完整程式先顯示四筆訂單，再檢查一個索引：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/conditions.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example conditions --locked
```

```text
小訂單=180，門檻訂單=300
大訂單=360，空訂單=0
索引無效，這次不讀取元素
```

80×2 的小計是 160，加 20 得 180；100×3 恰好達到 300，運費為零；120×3 得 360，也免運。數量零則在第一個 if 直接回傳 0，沒有再走運費計算。

## 多個條件依序檢查：else if

若運費改為「至少 500 免運、至少 300 收 10、其他收 20」，需要超過兩條路徑。`else if` 會在前一條件不成立時，才檢查下一個條件；第一個成立的分支執行後，就不再檢查其餘分支。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/conditions_answer.rs:tiers"
```

先檢查 500 很重要：500 也符合至少 300；若先用 300 的條件接走，它就會被收 10 元。這裡的順序直接表達「先處理最高門檻，剩下的再處理中間門檻」。

## 練習與完整答案

保留上面的三段運費規則，自己寫出一個完整程式，分別顯示小計 299、300、499、500 的運費。先預測答案：20、10、10、0。完成後再對照整份來源；它沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/conditions_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example conditions_answer --locked
```

299 與 300 檢查第一個門檻邊界，499 與 500 檢查第二個；不能只選一筆遠離門檻的資料，就推定包含等號的規則已經寫對。

[下一節：for 與 range](02-03-lesson.md)
