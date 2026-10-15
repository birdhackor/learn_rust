# 4.2 用 match 選分支，涵蓋每種情況

現在能保存三種 Quote，卻還不能把一個未知形態的 Quote 直接當金額相乘。要先分辨它是哪一種，再使用該形態附帶的資料。先用已懂的數字看分支如何選，再回到報價。

## 先用數字看 pattern 與 arm

**pattern**（模式）描述要符合的值或資料形狀。**match** 將一個值依序與各個 pattern 配對，只執行第一個符合的分支；每個 `pattern => expression` 稱為一個 **arm（分支）**。`=>` 左側是要符合的 pattern，右側是選中後執行的工作。

match 本身是 expression，結果來自被選中 arm 的 expression。

這次選定的運費規則是：等級 0 收 20，等級 1 收 10，其餘等級收 0：

`0`、`1` 是 literal pattern，只符合該數字。`_` 則符合任何值，但不建立名稱；放在最後表示前面沒有選中的其餘情況。它若放在最前面，所有值都先被接走，後面的分支就沒有機會執行。

```rust
--8<-- "examples/intro-workspace/patterns/examples/match_numbers.rs:function"
```

與 if 不同，match 後面不要求 bool，而是直接提供要配對的值。這裡的 `tier` 是 i32；例如 1 不符合第一個 pattern 0，符合第二個 pattern 1，因此交出 10，不再執行 `_` 那個 arm。

這裡各 arm 都交出 i32，整個 match 又是函式尾端 expression，所以成為運費的回傳值。

### 數字版本的完整程式

本頁各範例使用同一份完整設定：

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/match_numbers.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example match_numbers --locked
```

```text
等級0的運費=20
等級1的運費=10
等級2的運費=0
```

## 形態符合時，替裡面的資料取名稱

Quote 的 pattern 沿用建立值時的外框，但用途反過來：`Quote::Amount(amount)` 表示「符合 Amount，並把裡面的數字命名為 amount」。`amount` 是在這個 arm 建立的變數，只供該分支使用，不是要求金額等於一個名叫 amount 的既有值。

`Quote::Rejected { code }` 則符合有命名欄位的 Rejected，並用欄位同名的 code 接收數字；它是 `code: code` 的簡寫。Missing 沒有附帶資料，不需要在 pattern 多列名稱。

```rust
--8<-- "examples/intro-workspace/patterns/examples/match_quotes.rs:show"
```

每個 arm 都顯示一行，得到的都是 unit，所以這次 match 完成顯示工作，沒有交出額外金額。若 arm 有多項工作，就用 `{ ... }` 組成 block；其尾端值仍決定該 arm 的結果。

`show_quote(quote: Quote)` 接手完整 Quote，延續上一章的 owned 參數規則。本章稍後再比較借用 enum 的 pattern。

### 報價版本的完整程式

```rust
--8<-- "examples/intro-workspace/patterns/examples/match_quotes.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example match_quotes --locked
```

```text
目前沒有報價
報價=0
報價=120
拒絕報價，代碼=37
```

第二筆 Amount(0) 選中金額分支，沒有被當成 Missing。程式先保留情況，再取出資料，現在就能區分原本只用數字 0 時會混在一起的兩種意思。

## 涵蓋情況，與結果型別，各自都要成立

match 必須**完整涵蓋**要配對型別的可能情況，英文稱 **exhaustive**。下面完整失敗例的 Quote 簡化為兩種形態，卻只處理 Amount：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/missing_variant.rs"
```

放入獨立練習專案的 `src/main.rs`，執行 `cargo check`，錯誤 **E0004** 指出 Missing 未涵蓋。即使某次呼叫只打算傳 Amount，接受 Quote 的函式仍須處理其型別允許的 Missing。修正前先決定缺報價時要做什麼，再加入相應 arm；任意補上 0 可能又把兩種意思混起來。

下一份完整失敗例有 `_` 接住其餘數字，卻讓一個分支交出 i32，另一個交出 bool：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/arm_type.rs"
```

它會得到型別不符的 **E0308**。完整涵蓋沒有保證結果型別相容；要將 match 的結果當運費使用，各個交出結果的分支就都要提供相容的數字型別。

## 練習與完整答案

沿用上一節的 Shipment。寫 `ready_count(shipment: Shipment) -> i32`：Ready 交出 count；Waiting 對目前可出貨總數貢獻 0。原始資料仍以 Waiting 保存情況，0 是這項**統計工作**選定的結果。

完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/match_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example match_answer --locked
```

結果為 `等待的統計貢獻=0，可出貨=3`。這次需要為兩種形態各自決定答案，所以明寫兩個 arm。下一節把剛才「替內部資料命名」的工作，延伸到 tuple、array 與 struct。

[下一節：pattern 與解構](04-03-lesson.md)
