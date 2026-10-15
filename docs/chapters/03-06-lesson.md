# 3.6 impl 與 method：把訂單操作放回型別

目前 subtotal、set_count 都是獨立函式，第一個參數告訴我們它處理哪一筆訂單。如果希望從訂單寫出 `order.subtotal()`、`order.add_count(1)`，可以把這些操作放進 Order 的 **impl 區塊**。借讀、借改、接手的規則仍然相同。

## &self 讓 method 借讀呼叫者

`impl Order { ... }` 為 Order 定義操作。在其中，`Self` 代表這個 Order 型別。第一個參數若是 **self 參數**，這個函式就是 method，可以用 `值.方法名(...)` 呼叫。

先只需要讀單價與數量：`&self` 是 `self: &Self` 的簡寫，代表借讀呼叫方法的那筆 Order。

```rust
--8<-- "examples/intro-workspace/ownership/examples/methods.rs:read"
```

在 `order.subtotal()` 中，order 提供 self 那份借用，不需要再列進括號。方法內的 self 便是 reference；讀欄位時仍可直接用 `self.unit_price`。呼叫完成後，呼叫者保有原訂單。

## &mut self 修改呼叫者

要把數量加上 additional，方法需要修改借用。`&mut self` 是 `self: &mut Self` 的簡寫：

```rust
--8<-- "examples/intro-workspace/ownership/examples/methods.rs:write"
```

呼叫寫成 `order.add_count(1)`；order 必須宣告成 mut。這次呼叫需要獨占借用，仍受上一節的重疊使用規則限制。

## self 接手整筆值

若希望操作結束後交出各欄位，而呼叫者不再保留原訂單，就使用 `self`，即 `self: Self`：

```rust
--8<-- "examples/intro-workspace/ownership/examples/methods.rs:take"
```

這裡回傳第一章學過的 tuple `(i32, i32)`。`order.into_parts()` 把 order move 給方法，方法再交出兩個數字。因此呼叫之後不能繼續使用原 order。方法名稱不會自動決定 ownership；實際契約要看 self 參數型別。

## associated function 建立第一筆值

建立訂單時還沒有 order 可當 self，因此把建立函式寫在 impl 內，但不設 self 參數。這種與型別連在一起的函式稱為 **associated function**。

回傳型別 `Self` 就是 Order，`Self { ... }` 建立 Order 值。當欄位名和變數名相同，可把 `unit_price: unit_price` 簡寫成 `unit_price`，count 也是如此：

```rust
--8<-- "examples/intro-workspace/ownership/examples/methods.rs:new"
```

呼叫時用型別名稱與 `::`：`Order::new(80, 2)`。這裡 `new` 是我們選的函式名，Rust 不會自動替每個 struct 產生這個函式；建立時仍可使用第 3.1 節的欄位寫法。

## 完整程式：先建立、借讀、借改，最後接手

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/methods.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example methods --locked
```

```text
修改前=160，修改後=240
接手後交出的單價=80，數量=3
```

`Order::new` 先建立 owned 值；兩次 subtotal 借讀，中間 add_count 借改，最後 into_parts 接手。依照這個順序，每一份借用都在後續動作前完成。

## &self 不能交出借來的整筆 Order

下面故意以 &self 借讀，卻想把整筆值交出去：

```rust
--8<-- "examples/intro-workspace/ownership/fixtures/compile-fail/move_from_borrow.rs"
```

`*self` 指到借來的 Order。這個自訂型別沒有 Copy，不能經由 shared reference 把它 move 出來，編譯器回報 **E0507**。借讀並未取得接手權。若原本工作就是要交出整筆值，方法的 self 參數應表明它接收 owned 的 Self。

## 練習與完整答案

新增 `reset_count(&mut self)`，只把 count 歸零。用單價 80、數量 3 的訂單驗證：原小計是 240，歸零後 count 是 0、unit_price 仍是 80，小計是 0。

```rust
--8<-- "examples/intro-workspace/ownership/examples/methods_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example methods_answer --locked
```

通過四項數值檢查後，顯示 `清空後數量=0，單價=80，小計=0`。方法把訂單相關操作放在同一個型別旁，參數則繼續明確交代每次操作需要的權限。

[下一節：slice](03-07-lesson.md)
