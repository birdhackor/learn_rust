# 2.5 用 method 表達資料能做的工作

我們已經有 `Order`，也有借用它計算小計的函式。當資料的工作增加，想找「訂單如何算小計、如何增加數量」時，可以把這些行為放在型別旁邊。計算和 borrowing 規則沿用前文，這一節改變的是行為如何組織與呼叫。

## 從函式參數走到 method 的 self

1.7 曾用 `prices.len()` 呼叫 array 上的 method。**method（方法）** 是針對某個值呼叫的函式；在 `order.subtotal()` 中，點號前的 `order` 就是要處理的那筆資料。

`impl Order { ... }` 定義與 `Order` 相關的行為。`impl` 來自 implementation，這個 block 讓裡面的函式與這個型別關聯，而不是像普通自由函式那樣各自散開。

method 的第一個參數名為 **`self`**，代表呼叫時提供的那個值。在 `impl Order` 中，大寫的 **`Self`** 表示 `Order` 這個型別，小寫的 `self` 則是參數名。`&self` 是 `self: &Self` 的簡寫，在這裡等同於接受 `&Order`。因此前節的 `subtotal(order: &Order)` 可以改成 method `subtotal(&self)`，body 則用 `self.unit_price` 與 `self.count`。

點號呼叫會依 receiver，也就是這個第一參數的形式，建立適當的借用或交出值。對 `&self` 的 method，`order.subtotal()` 借用原訂單；對 `&mut self` 的 method，則借用以修改；對 `self` 的 method，則交出完整值。它不會因為換成 method 語法就避開 ownership 規則。

## 三種 receiver 回應三種需要

| 工作 | receiver | 為什麼這樣選 |
| --- | --- | --- |
| 只讀欄位算小計 | `&self` | 呼叫者還要保留訂單，函式也不需要修改它。 |
| 修改數量 | `&mut self` | 修改同一筆訂單，完成後由呼叫者繼續使用。呼叫者的變數需為 mutable。 |
| 接手訂單，交出單價與數量這個 tuple | `self` | 這項工作明確交出原訂單，呼叫者後面改用回傳的兩個值。 |

method 除了 receiver，還能有普通參數。例如 `add_count(&mut self, additional: i32)` 的 `additional` 表示增加幾件；呼叫寫 `order.add_count(1)`，不再把 `order` 重複寫進括號。

## associated function 不需要既有實例

要建立訂單時，還沒有一筆可提供給 `self` 的 `Order`。我們可以在同一個 `impl` 中定義沒有 `self` 的函式，稱為 **associated function（關聯函式）**。它和型別相關，但不是透過某個既有實例呼叫的 method。

這裡寫 `new(unit_price: i32, count: i32) -> Self`，產生新的 `Order`。`new` 是常用名稱，不是 Rust 特殊關鍵字；呼叫寫 `Order::new(80, 2)`。`::` 在此從型別找其關聯函式，與上一章從 module 找函式的路徑表示相接。

建立值時，若已有變數與欄位同名，可以簡寫 `Self { unit_price, count }`。其中 `unit_price` 等同 `unit_price: unit_price`，第一個角色是欄位名，第二個是提供值的變數；這只是省略重複名稱，不是另一次計算。

現在把只讀、修改、接手與建立四種工作放在同一個 `impl`，再從 `main` 依序呼叫：

```rust
--8<-- "examples/ch02-ownership/examples/methods.rs"
```

```bash
cargo run -p ch02-ownership --example methods --locked
```

```text
修改前=160，修改後=240
接手後交出的單價=80，數量=3
```

`subtotal` 兩次都只讀資料。`add_count` 把原訂單數量由 2 改為 3。最後 `into_parts` 接手 `Order`，把兩個欄位交成 tuple；後面使用 `price` 與 `count`，沒有再用已交出的 `order`。`into_...` 常用來命名接手完整值、產生另一種結果的操作，本例依簽名中的 `self` 判斷行為，不能只憑名稱推測。

## 能讀到欄位，不代表能從借用交出完整值

前面的 `&self` method 能用 `self.count` 取得 `i32`，因為數字具備 Copy。若改成回傳完整 `Order`，直接寫 `*self` 便想從借來的位置 move 它；我們的 `Order` 沒有 Copy，shared reference 也沒有取得所指資料的 ownership，因此不能這樣交出。

下面是故意無法編譯的 fixture：`into_order` 宣告只借用，body 卻嘗試交出整筆值。

```rust
--8<-- "examples/ch02-ownership/fixtures/compile-fail/move_from_borrow.rs"
```

錯誤碼 `E0507` 表示不能 move 出 shared reference 後面的值。若工作本來就是接手訂單，receiver 應選 `self`；若只是借用讀取，則留在可讀欄位與計算結果的範圍。這個差別來自值的能力與借用契約，並非所有使用 `*` 的 expression 都錯。

## 練習：把數量清空，單價保留

在自己的 `Order` impl 加入 `reset_count`，將原訂單的數量改成 0；它不應接手並結束整筆資料，也不應改單價。先選 receiver，再寫 body。

以單價 80、數量 3 驗證：呼叫前小計 240；呼叫後仍能讀取訂單，數量 0、單價 80、小計 0。用已教的數值 assertion 分別確認，避免只看到總額 0，卻沒有發現單價也被改成了 0。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch02-ownership/examples/methods_solution.rs)，或執行 `cargo run -p ch02-ownership --example methods_solution --locked`。

你已能把 ownership 的選擇放進自訂資料的操作介面。下一節回到數列，讓函式借用一段連續資料，而不被固定 array 長度綁住。
