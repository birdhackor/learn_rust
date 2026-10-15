# 1.6 函式、expression、block 與 scope

單價乘數量的規則會用在多筆訂單。直接寫兩次乘法也能算對，但規則改變時，就要找到每個重複位置。現在把這項工作命名為 `subtotal`，讓每次呼叫提供不同材料，規則只保存在一處。

## 先定義要收什麼、交出什麼

`fn subtotal(price: i32, count: i32) -> i32` 定義一個叫 `subtotal` 的函式。括號中的 `price`、`count` 是**參數**：這次呼叫收到資料後，在函式內使用的名稱。每個參數都要寫型別，逗號分開兩個參數。箭頭 `-> i32` 表示函式會交回一個 `i32`，叫作**回傳值**。

要交回乘法的結果，在函式最後寫 `price * count`，這一行**不加分號**：

```rust
--8<-- "examples/intro-workspace/steps/examples/functions.rs:subtotal"
```

`price * count` 是 **expression**（表達式）：能算出一個值。相對地，`let total = ...;` 是 **statement**（陳述式）：完成宣告，不能把整個 `let` 當成乘法那樣的數值使用。

函式的 `{ ... }` 是一個 **block**，也就是大括號圈起的一段工作。block 最後的不帶分號 expression 會成為它的值；函式本體的這個值就交回呼叫者。因此上面兩個參數決定這次乘法的材料，尾端決定交出的答案。

## 呼叫把資料交給參數

呼叫寫成 `subtotal(120, 3)`，括號中的實際輸入按位置對應：`price` 得到 120，`count` 得到 3。呼叫算出 360 後，這個值可以成為另一個變數的初始值。

先補上 `show_total`：它收到一個 `i32`，只負責顯示，不需要交回額外數字。

```rust
--8<-- "examples/intro-workspace/steps/examples/functions.rs:show"
```

Rust 用 **unit 型別 `()`** 表示沒有額外內容的結果，這個型別只有一個值，也寫作 `()`。函式省略 `-> ...` 時，回傳型別就是 `()`；上面的 block 沒有尾端值，因此結果是 unit。它仍是值與型別，不能拿來當 `i32` 相加。第一個程式中的 `main` 也是這種情況。

現在 `main` 就能呼叫已定義的兩個函式：

```rust
--8<-- "examples/intro-workspace/steps/examples/functions.rs:calls"
```

第一行呼叫完成後，`first` 才取得 360；第二行用 80、2 做新一次呼叫，得到 `second` 的 160。接著兩次 `show_total` 各顯示一個總額。參數名稱不必和呼叫端的名稱一樣，每次呼叫都使用本次提供的材料。

## 完整的兩筆訂單程式

```rust
--8<-- "examples/intro-workspace/steps/examples/functions.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example functions --locked
```

```text
總額=360
總額=160
```

程式仍從 `main` 開始，函式定義本身不會立刻執行。這裡把兩個被呼叫的函式寫在前面方便閱讀，寫在 `main` 後面也可以。在同一檔案的這個安排中，Rust 能先找到函式定義，再檢查呼叫是否提供正確數量與型別的材料。

## 多一個分號，結果會不同

`price * count;` 仍完成乘法，但分號把它變成一項不留下該數值的工作。若後面沒有其他尾端 expression，block 的結果就成為 unit。

下面完整的預期失敗程式，簽名承諾回傳 `i32`，本體卻得到 `()`：

```rust
--8<-- "examples/intro-workspace/steps/fixtures/compile-fail/return_semicolon.rs"
```

編譯錯誤的重點是 `expected i32, found ()`。需要修的是函式最後的分號；`let` 宣告與已完成的顯示工作仍要以分號結束。

## 內部 block 也能交出結果

現在計算含 20 元運費的總額。先在內部 block 命名小計，再交出小計加運費的結果：

```rust
--8<-- "examples/intro-workspace/steps/examples/blocks.rs:block"
```

最外面的 `;` 結束 `let total = ...;` 宣告；內部最後的 `item_total + 20` 不加分號，讓 380 交到 `total`。這個 block 不是新函式，也沒有函式呼叫。

block 同時界定名稱的 **scope**（作用範圍）。內部可以讀到外層的 `price`、`count`；內部宣告的 `item_total` 從宣告後到該 block 結束可用。離開後，外層可使用收到的 `total`，卻不能再以 `item_total` 這個名稱取得小計。

scope 也決定 shadowing 何時結束：

```rust
--8<-- "examples/intro-workspace/steps/examples/blocks.rs:nested_shadow"
```

內層新 `count` 是 4；內層結束後，外面的 `count` 仍是 3。shadowing 沒有改掉外層 binding。

這兩個 block 組成下面的完整程式：

```rust
--8<-- "examples/intro-workspace/steps/examples/blocks.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example blocks --locked
```

輸出依序是含運總額 380、內層數量 4、外層數量 3。若想確認 scope 限制，下面是完整的預期失敗程式；最後一行無法找到 `item_total`：

```rust
--8<-- "examples/intro-workspace/steps/fixtures/compile-fail/scope.rs"
```

這是編譯時對名稱範圍的檢查。下一節再觀察函式呼叫前、中、後的資料如何銜接，不能只從名稱能否使用，推定實際記憶體一定放在哪裡。

## 練習與完整答案

定義 `with_shipping`，收到單價、數量、運費三個 `i32`，回傳完整總額。讓它呼叫已完成的 `subtotal`，再加運費；用 `120, 3, 20` 與 `80, 2, 20` 取得 380、180。

```rust
--8<-- "examples/intro-workspace/steps/examples/functions_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example functions_answer --locked
```

不同訂單的材料從呼叫位置提供；共同的乘法留在 `subtotal`，加運費的工作留在 `with_shipping`。完整答案沒有依賴其他檔案的函式。

[下一節：程式執行時，電腦如何保存資料](01-07-lesson.md)
