# 3.4 mutable reference：修改借來的資料

shared reference 能讀訂單，不能經由它把數量加一。現在希望函式完成修改後，呼叫者直接看見同一筆訂單的新數量，仍不必交出整筆值。這需要 mutable reference。

## &mut 借出修改權限

**mutable reference** 提供對所借資料的獨占存取。`&mut order` 建立這種借用，型別是 `&mut Order`；原變數必須先宣告為 mutable。當這份借用仍在使用時，不能從其他衝突途徑存取同一資料，第 3.5 節再逐行判斷區間。

函式參數寫 `order: &mut Order`，便能經由借用修改原欄位：

```rust
--8<-- "examples/intro-workspace/ownership/examples/mutable_references.rs:modify-order"
```

呼叫寫 `add_one(&mut order)`。函式完成後，main 讀到的數量是 4，單價仍是 120。函式沒有另建一筆訂單，也沒有接手原訂單的 ownership。

對單一數字，先用 `*` 找到原值，再修改：

```rust
--8<-- "examples/intro-workspace/ownership/examples/mutable_references.rs:modify-count"
```

`*reference += 1` 把借來的數字由 3 改為 4。這個 reference binding 本身沒有寫 `mut`，因為它沒有換成另一份 reference；改的是它借來的資料。

## 第一份完整程式

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/mutable_references.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example mutable_references --locked
```

```text
原訂單改成數量=4，單價=120
透過 mutable reference 得到的數字=4
```

## let mut reference 改的是 binding

反過來，`let mut reference: &i32` 允許重新指派這個變數，讓它保存另一份 shared reference；`&i32` 仍然只允許借讀。下面的完整程式先借 first，再把 reference 改成借 second：

```rust
--8<-- "examples/intro-workspace/ownership/examples/reference_binding.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example reference_binding --locked
```

```text
第一次讀=3
換一個借用對象後=8
原資料仍是 3 和 8
```

前兩行不同，來自這個 binding 保存了不同的 reference，不是把第一個數字由 3 改成 8。宣告名稱前的 `mut` 和型別中的 `&mut` 各有對象：前者允許重新指派 binding，後者允許經由借用修改所指資料。

下面是完整的預期編譯失敗例，故意經由 shared reference 修改原數字：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/shared_write.rs"
```

放入獨立練習專案後執行 `cargo check`，會得到 **E0594**：不能透過 `&` reference 指派。只在 reference 名稱前加 mut，不會把它的 &i32 型別變成 &mut i32。

## 練習與完整答案

寫 `set_count`，接受 `&mut Order` 與新的 i32 數量，修改原訂單；再寫接受 `&Order` 的 subtotal，只讀欄位算小計。以單價 80、數量 2，先算 160，將數量設成 3，再算 240。

先依工作選參數：設定數量需要修改借用，計算小計只需 shared borrow。呼叫者始終持有這筆訂單；它自己要宣告成 mut，才能提供修改借用。

```rust
--8<-- "examples/intro-workspace/ownership/examples/borrow_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example borrow_answer --locked
```

兩項數值檢查通過後顯示 `修改前=160，修改後=240`。這裡的借讀、修改依序完成；下一節再確認，編譯器如何根據後續使用判斷哪些操作會衝突。

[下一節：borrowing、lifetime 與 scope](03-05-lesson.md)
