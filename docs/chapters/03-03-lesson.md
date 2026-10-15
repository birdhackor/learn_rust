# 3.3 shared reference 與 dereference：借讀原資料

上一節的 subtotal 接手整筆 Order，只交回數字；若呼叫者還想用訂單，就得另行交回它。現在我們希望函式只讀欄位算小計，呼叫之後原訂單仍留在 main。要改的是函式取得資料的方式，乘法規則保持相同。

## reference 借用既有的值

**reference**（參照）是用來借用既有資料的值。它保存找到資料的資訊，並受 Rust 的有效性與存取規則檢查；可以接回第 1.7 節 pointer「到哪裡找」的角色，但不是直接取得原資料的 ownership。

`&count` 建立對 count 的借用，結果型別為 `&i32`。**borrowing** 就是建立這種借用的動作。在 expression 裡的 `&` 建立 reference；型別中的 `&i32` 則說明它借用一個 i32，兩者位置與角色不同。

這種 `&i32`、`&Order` 稱為 **shared reference**。它可與其他 shared borrow 一起讀取；對本章的普通數字欄位，不能透過它修改原資料。借用使用期間，原資料也必須仍有效；第 3.5 節會具體追蹤這個範圍。

圖中 order 仍持有唯一一筆 Order；reference 的箭頭只表示借用，沒有產生第二份訂單：

![order 仍持有 Order，reference 借用同一筆資料](../assets/ch02/reference-owner.svg)

## 參數改成 &Order，呼叫時提供借用

`subtotal(order: &Order)` 接受 reference。函式只讀兩個數字欄位、交回小計。讀 `order.count` 時，欄位存取會自動跟隨這份 reference 找到原訂單；不是讓 Order 突然具備 Copy。

```rust
--8<-- "examples/intro-workspace/ownership/examples/shared_references.rs:borrow-function"
```

main 建好 Order 後，以 `&order` 建立借用並呼叫：

```rust
--8<-- "examples/intro-workspace/ownership/examples/shared_references.rs:borrow-call"
```

參數沒有取得原訂單的 ownership，因此函式結束後，main 仍能讀 order.count。reference 自己用完，也不會因此清理它借用的原資料。

## * 找到 reference 所指的值

`*reference` 是 **dereference**，也就是解參照：沿 reference 找到所指的值。對 `i32`，可以把讀到的數字 Copy 出來：

```rust
--8<-- "examples/intro-workspace/ownership/examples/shared_references.rs:dereference"
```

`count_reference` 是一份借用，`copied_count` 則是獨立的數字 3。原 count 仍是 3。`*` 放在一個 expression 前表示 dereference；放在兩個數字中間，如 `price * count`，則仍是乘法。

欄位存取也可以明寫成 `(*order).count`：先 dereference，再讀欄位。括號讓整個 `*order` 成為取欄位的對象；本節完整函式採用自動跟隨 reference 的 `order.count`。

## 完整範例與執行

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/shared_references.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example shared_references --locked
```

```text
借用算出的小計=360，原數量=3
讀出的數字=3，原數字=3
```

與上一節相比，乘法答案相同；改變的是呼叫者不必先交出再收回訂單，因為這項工作只需要讀取。

## 練習與完整答案

寫 `total_with_shipping(order: &Order, shipping: i32) -> i32`，只借讀訂單，回傳商品小計加運費。以單價 80、數量 2、運費 20，確認結果 180，呼叫後原數量仍是 2。

先確定函式沒有修改或接手訂單的需要；參數用 &Order，呼叫位置就提供 &order。完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/ownership/examples/shared_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example shared_answer --locked
```

成功後顯示 `含運總額=180，原數量=2`。接下來若工作要修改同一筆訂單，就需要另一種借用權限。

[下一節：mutable reference 與變數的 mut](03-04-lesson.md)
