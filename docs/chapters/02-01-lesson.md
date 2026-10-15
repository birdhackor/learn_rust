# 2.1 比較、bool 與短路邏輯

第一章已能算出商品小計。現在想回答另一種問題：「小計達到 300 元了嗎？」答案不是另一個金額，而是 `true` 或 `false`。先學會算出這個答案，下一節才讓程式依答案選擇不同工作。

## 比較把數字變成 bool 答案

**比較運算**檢查兩個值的關係，得到 `bool`。本節比較已知的 `i32`：

| 寫法 | 何時得到 true |
| --- | --- |
| `subtotal == 300` | 等於 300 |
| `subtotal != 300` | 不等於 300 |
| `subtotal < 300` | 小於 300 |
| `subtotal <= 300` | 小於或等於 300 |
| `subtotal > 300` | 大於 300 |
| `subtotal >= 300` | 大於或等於 300 |

`=` 用來提供初始值或重新指派；`==` 才是相等比較。現在刻意讓小計恰好是 300，觀察包含等號與不包含等號的差異：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/comparisons.rs:comparisons"
```

三行輸出分別是 `true/false`、`false/true`、`false/true`。`>=` 的等號使恰好 300 也符合門檻；若將小計改成 299，`at_least` 就會變成 `false`。`println!` 的 `{名稱}` 也能直接顯示布林值。

## 組合幾個條件

單價與數量都非負，才算這次材料通過基本檢查。`&&` 表示「兩邊都要為 true」；`||` 表示「至少一邊為 true」；`!` 放在一個布林值前面，將 true 與 false 對調。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/comparisons.rs:logic"
```

這次 `valid` 是 true，因為 120 與 0 都非負；`empty` 是 true，所以 `!empty` 得到 false。數量非負與有商品是兩項不同檢查，不能因第一項通過就推定第二項也通過。

`&&` 與 `||` 兩側都必須得到 `bool`。要檢查金額介於 0 與 300，寫 `amount >= 0 && amount <= 300`；不要把 Python 的鏈式比較 `0 <= amount <= 300` 原樣搬過來。混合條件時可加括號，明寫要先組合的部分，例如 `ready && (paid || free_order)`。

## 短路：右側不一定執行

組合條件時，Rust 先算左側。對 `&&` 而言，左側已經是 false，整體就不可能為 true，因此跳過右側；對 `||` 而言，左側已是 true，整體已成立，也跳過右側。這稱為**短路判斷**。

要看見右側究竟有沒有執行，我們沿用函式的做法，讓數量檢查先印一行，再交出 bool：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/comparisons.rs:count-check"
```

`-> bool` 和第一章的 `-> i32` 一樣，指定函式回傳的型別。這裡尾端 `count > 0` 就提供該布林值；顯示那一行只是為了追蹤這次有沒有呼叫函式。

再把這項檢查放到右側：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/comparisons.rs:short-circuit"
```

`both_checks` 要求單價非負且數量為正。`either_check` 則用來觀察另一項組合規則：已備妥或數量為正，只要其中一項成立就回傳 true。它們是兩項分開的示範規則，不是在這裡決定最終出貨政策。

完整程式呼叫四次，先預測哪兩次會出現「檢查數量」：

| 呼叫 | 左側答案 | 是否執行右側 |
| --- | --- | --- |
| `both_checks(-1, 2)` | false | 不執行，整體為 false |
| `both_checks(120, 2)` | true | 執行，數量為正，整體為 true |
| `either_check(true, 0)` | true | 不執行，整體為 true |
| `either_check(false, 0)` | false | 執行，數量不是正數，整體為 false |

## 完整範例與執行

本章前五節的程式放在 `control-steps/`，package 名稱是 `intro-control-steps`。它與第一章的 `steps/` 一樣，沒有 library；每份範例有自己的 `main`。完整 member manifest 如下：

```toml
--8<-- "examples/intro-workspace/control-steps/Cargo.toml"
```

下面包含本節全部函式與呼叫；指令仍在教材 repo 根目錄執行：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/comparisons.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example comparisons --locked
```

```text
等於=true，不等於=false
小於=false，小於等於=true
大於=false，大於等於=true
材料非負=true，有商品=false
A：兩項都要成立，單價為負
A的結果=false
B：兩項都要成立，單價非負
檢查數量=2
B的結果=true
C：任一項成立，已備妥
C的結果=true
D：任一項成立，尚未備妥
檢查數量=0
D的結果=false
```

結果不只顯示四個 bool，也顯示只有 B、D 呼叫了右側函式。短路改變的是哪些工作實際發生，不只是最後寫出哪一個布林值。

## 練習與完整答案

定義 `can_ship(price: i32, count: i32, ready: bool) -> bool`：單價非負、數量為正、已備妥，三項都成立才能出貨。再定義 `needs_preparation(ready: bool) -> bool`，將是否備妥反轉為是否尚需備貨。使用下列完整答案前，先預測 `(80, 2, true)`、`(80, 0, true)`、`(-1, 2, true)` 三次出貨檢查。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/comparisons_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example comparisons_answer --locked
```

三次出貨答案依序是 true、false、false；尚未備妥時，`needs_preparation(false)` 為 true。這份答案沿用同頁 manifest，沒有其他來源檔案。

[下一節：if、else 與提早回傳](02-02-lesson.md)
