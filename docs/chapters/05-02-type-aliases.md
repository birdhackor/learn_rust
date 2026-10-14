# 5.2 為既有型別取另一個名字

前節的驗證函式交回 `Result<i32, CountError>`。同樣的型別若在數個函式重複出現，可以用一個短名稱表示，讓閱讀焦點留在每項工作需要什麼、交出什麼。

**type alias（型別別名）** 是既有型別的另一個名稱。`type CountResult = Result<i32, CountError>;` 宣告新名字；`type` 表示這是型別名稱的安排，等號右邊仍是原型別，不是在執行時建立另一個值。

`pub type` 讓 library 外也能使用這個名字。這次在 `aliases` module 先取名字，再讓函式使用它：

```rust
--8<-- "examples/ch05-errors/src/aliases.rs"
```

`count_with_alias` 仍呼叫同一個 checked_count，沒有改成功與失敗規則。下面先用 CountResult 接收結果，再改用完整型別名稱接收同一個值，不需轉換：

```rust
--8<-- "examples/ch05-errors/examples/aliases.rs"
```

```bash
cargo run -p ch05-errors --example aliases --locked
```

輸出是 `兩種型別名稱都表示=Ok(3)`。這證明的是兩個名稱表示同一個型別，不是把原結果解析成另一種資料。

## 別名沒有新增驗證行為

若把 i32 取名 CountNumber，−1 仍是合法的 i32 值；名字不會把前節 0 到 100 的驗證規則加入型別：

```rust
--8<-- "examples/ch05-errors/examples/alias_is_name.rs"
```

執行 `cargo run -p ch05-errors --example alias_is_name --locked`，仍會顯示 −1。需要驗證就得實際執行驗證工作，不能用名字代替。

下一節會見到 `fmt::Result`：它不是另一個陌生的結果容器，而是標準函式庫替特定成功／失敗型別取的 alias。先讀懂這個關係，才知道格式化函式承諾交回什麼。

## 練習：替兩個數字的組合命名

宣告 `PriceAndCount`，表示 `(i32, i32)`；第一項為單價，第二項為數量。寫 subtotal 接收這個名字表示的 tuple，交回相乘結果。

用單價 80、數量 3 得到 240，再把同一份資料指定給型別為 `(i32, i32)` 的變數，確認不需要轉換。這個名稱只改善閱讀，位置的角色與 tuple 的規則仍要由介面說清楚。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch05-errors/examples/alias_solution.rs)，或執行 `cargo run -p ch05-errors --example alias_solution --locked`。
