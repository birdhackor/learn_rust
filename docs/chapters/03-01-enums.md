# 3.1 用 enum 表示一筆資料的不同情況

上一章的 `Order` 同時有單價與數量。現在要接收一筆報價，情況可能是「尚未提供」、「有金額」或「被拒絕，附上一個代碼」。如果只存一個數字，把沒有報價寫成 0，就無法區分「缺少資料」與「真的報價為零」。我們需要讓情況本身也是資料。

## 一個型別，幾種互斥形態

**enum** 是列出幾種可能形態的自訂型別；每種形態叫 **variant**。一個 enum 值在某個時刻只屬於其中一種 variant。struct 表示一筆資料同時有那些欄位，enum 則表示這筆資料是其中哪一種情況。

本例的型別叫 `Quote`，有三種 variant：

| variant 的形式 | 一個具體值 | 保存的資訊 |
| --- | --- | --- |
| `Missing` | `Quote::Missing` | 沒有報價，沒有額外資料 |
| `Amount(i32)` | `Quote::Amount(120)` | 有報價，附帶一個 `i32` 金額 |
| `Rejected { code: i32 }` | `Quote::Rejected { code: 37 }` | 被拒絕，附帶名為 code 的欄位 |

`enum Quote { ... }` 定義這些形態，各 variant 以逗號分隔。它們都屬於同一個 `Quote` 型別，`Quote::` 指出正在建立哪個型別的 variant。小括號與花括號分別表示位置資料與命名欄位；沒有附帶資料的 variant 就不加這兩種外框。

因此，接受 `Quote` 的函式能收到上述三種情況，但不能把 `Quote::Missing` 直接拿來乘數量。先要知道是哪一種形態，才知道有什麼資料可用。

## match 先分辨形態，再取出其中資料

**pattern（模式）** 描述要符合的值形狀，也可以為其中的資料取名稱。`Quote::Amount(amount)` 作為 pattern 時，表示「只符合 Amount，將裡面的數字命名為 amount」。它和建立值的 `Quote::Amount(120)` 使用相同外框，但現在是在拆出已有的資料。

`match 值 { pattern => 工作, ... }` 依序找符合的 pattern，只執行第一個符合的分支。每個 `pattern => 工作` 稱為一個 **arm（分支）**；`=>` 把條件形狀與要做的工作分開。`match` 必須涵蓋所有可能情況，編譯器會檢查是否遺漏。

命名欄位的 pattern 寫 `Quote::Rejected { code }`，把名為 code 的欄位取出並命名為 code；這是和 struct 欄位對應的解構。pattern 也能用數字 literal；`_` 則符合任何值但不綁定名稱，作為其餘情況的分支時應放最後。

下面的 `show_quote` 接收完整 `Quote`，所以呼叫會交出該值的 ownership。三個分支都只顯示一行，回傳前文已教的 unit `()`：

```rust
--8<-- "examples/ch03-data/src/main.rs"
```

從 repo 根目錄執行本章 package：

```bash
cargo run -p ch03-data --locked
```

```text
目前沒有報價
報價=120
拒絕報價，代碼=37
```

追蹤第二次呼叫：傳入的是 Amount 形態，pattern 取得其中的 120，只有 Amount arm 執行。不是把三項工作全部做完，也不是用數字真假選分支。第三次則取得命名欄位中的 37；一份 Quote 不會同時有兩種 variant 的附帶資料。

### 分支的結果也是值

`match` 是 expression，整個結果是選中 arm 的值。要回傳 `bool` 時，每個 arm 都交出 `bool`；不能一邊交出數字、一邊交出布林值。這延續 1.6 的 `if` expression 型別要求，只是選分支的材料改成 pattern。

以下故意遺漏 Missing，簽名雖然正確，卻不能通過編譯：

```rust
--8<-- "examples/ch03-data/fixtures/compile-fail/missing_variant.rs"
```

錯誤碼 `E0004` 指出 pattern 沒有涵蓋全部情況。修正時應先決定缺值要做什麼，再加入相應 arm；不是隨便補一個數字讓檢查通過。

## 借用 enum，pattern 取得的也會是借用

若呼叫者還需要原報價，就讓函式接收 `&Quote`。對 shared reference 做 `match` 時，可以沿用 `Quote::Amount(amount)` 的形狀，Rust 會借用符合的內部資料：這裡的 `amount` 是 **`&i32`**，不是一個新的 Quote，也不是接手那個金額。

因此 `*amount` 才是借來的數字。`i32` 有 Copy，讀出它作比較不會 move 整份 Quote。命名欄位的 `code` 同理是 `&i32`。這項 reference 的 pattern 對應，是本節新增的解構規則；仍須遵守上一章的 borrowing 與有效範圍要求。

下面把「金額非負」作為有可用報價的條件，讓 `match` 交出 `bool`。代碼只供顯示，本例不嘗試查詢外部拒絕原因：

```rust
--8<-- "examples/ch03-data/examples/borrow_patterns.rs"
```

```bash
cargo run -p ch03-data --example borrow_patterns --locked
```

你會看到同一筆 Amount 報價的兩次判斷都為 `true`；檢查 Rejected 時再顯示代碼 37。函式只借用，所以兩次呼叫使用相同的 `quote`。`match` 並不要求每次都交出原資料，參數與 pattern 的借用方式才決定這裡怎麼使用它。

## 只關心一種形態：if let 與 let else

有時工作只是「若有金額，就顯示它」。**`if let pattern = 值 { ... }`** 只在符合該 pattern 時執行 body，其他情況略過；也可以加 `else` 處理所有不符合的情況。它沒有要求你為每種 variant 各寫一個 arm，所以不適合取代那些本來就必須分別處理的情況。

另一個常見需求是：「沒有金額就先結束，有金額才繼續算運費。」**`let pattern = 值 else { ... };`** 在符合時把取出的名稱留在後面的 scope；不符合時執行 else，而且 else 必須離開目前這段流程。本例使用 `return;`，也就是從回傳 unit 的函式提早結束。結尾的分號結束這項 let 宣告。

這份小例只處理「有報價／缺值」，因此把 Quote 簡化為 Amount 與 Missing；拒絕代碼的分別處理仍由前面的完整版本示範。

```rust
--8<-- "examples/ch03-data/examples/branch_patterns.rs"
```

```bash
cargo run -p ch03-data --example branch_patterns --locked
```

```text
只關心報價=120
報價加運費=140
略過沒有報價的資料
```

`if let` 內的 amount 只在它的 body 使用；`let else` 的 amount 則在成功之後供下一行計算。對 Missing 呼叫時，函式在 else 提早結束，不會繼續加 20。兩者都透過 reference 借用資料，沒有延長原值的存活時間。

## 練習：等待與可出貨數量

定義 `Shipment`，包含無資料的 Waiting，以及有命名欄位 `count: i32` 的 Ready。寫 `ready_count(shipment: &Shipment) -> i32`：Waiting 對目前可出貨總數貢獻 0，Ready 則交出其 count。這裡的 0 是統計規則，原資料仍保存「等待」這種情況。

以 Waiting、Ready 的 count 3 驗證結果為 0、3，再借用同一筆 Ready 一次仍得到 3。只對回傳的數字使用 `assert_eq!`，不要直接比較整個自訂 enum。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch03-data/examples/enum_solution.rs)，或執行 `cargo run -p ch03-data --example enum_solution --locked`。下一節會把「有值／缺值」與「成功／失敗」這兩種經常需要的形態接到標準函式庫。
