# 5.1 泛型（generics）：共用資料形狀，替換其中型別

第 3 章的 Order 把欄位型別寫成 i32。現在換一個小需要：把一份資料放進只有 value 欄位的盒子，有時保存數字，有時保存 bool 設定。我們希望共用「保存輸入、之後交回」的規則，而每一個盒子仍有明確的欄位型別。

## 型別參數決定欄位採用哪個型別

**泛型（generics）**讓型別或函式定義使用**型別參數**。值參數收到的是 120、true 這種值；型別參數指定的是 i32、bool 這種型別。兩者不是同一類輸入。

`Holder<T>` 在型別名稱後的角括號內宣告一個型別參數 T，value 欄位採用 T。T 是常用名稱，不是 Rust 特別提供的一個具體型別：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_holder.rs:holder"
```

使用時填入具體型別：

| 寫法 | value 欄位的型別 | 可保存的例子 |
| --- | --- | --- |
| `Holder<i32>` | i32 | 120 |
| `Holder<bool>` | bool | true |

這是兩個不同的具體型別。`Holder<i32>` 建立後，不能在執行時把同一欄位任意改成 bool；泛型保留型別檢查，只讓定義中的型別位置可以共用。

## 泛型函式仍要交代輸入與輸出的對應

`hold<T>` 在函式名後宣告 T。參數 `value: T` 和回傳 `Holder<T>` 使用同一個 T，表示輸入型別就是結果盒中資料的型別。

函式只把原值放進盒子，不假定所有型別都能相乘或顯示。`Holder { value }` 沿用第 3.6 節的同名欄位簡寫，等同於把參數 value 放進名為 value 的欄位：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_holder.rs:hold"
```

保存時依實際型別做 Copy 或 move，規則沒有改變。接著用明確型別的變數接收兩次呼叫：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_holder.rs:calls"
```

第一行要求 `Holder<i32>`，第二行要求 `Holder<bool>`；各自的輸入也提供型別線索。省略接收變數上的註記時，編譯器仍可由這些線索推導，並不是讓盒子沒有型別。

## 在呼叫處指定型別

如果要直接在函式呼叫指定 T，寫 `hold::<i32>(80)`。`::<...>` 俗稱 **turbofish**：這段角括號指定型別參數，後面的普通小括號才提供值參數。

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_holder.rs:explicit"
```

這次明定 T 是 i32，值是 80。推導已足夠清楚時，不需要每次加這段指定。

## 第一份完整程式

本節與接下來三節的階段程式放在 `data-steps/`，package 名稱是 `intro-data-steps`。它沒有 library；每個 example 都有自己的完整定義與 main。完整設定如下：

```toml
--8<-- "examples/intro-workspace/data-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_holder.rs"
```

從教材 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example generic_holder --locked
```

```text
數字盒=120，布林盒=true
指定型別的數字盒=80
```

顯示與 assertion 都發生在型別已確定的位置：這裡的欄位是 i32 或 bool。hold 本身不必為每個型別各寫一份保存程式。

## 泛型 impl：盒子自己建立與交回值

接著把操作放到型別旁，像第 3.6 節的 Order::new。**`impl<T> Holder<T>`** 中，前面的 `<T>` 宣告這個 impl 要用的型別參數，後面的 `Holder<T>` 才使用它；impl 內的方法也能使用同一個 T。

這裡 Self 代表正在實作的 `Holder<T>`。`new(value: T) -> Self` 收到一份值，保存後交回盒子；`into_value(self) -> T` 接手整個盒子，將其中的值交回。它收到的是 owned 的 self，所以能交出欄位，不需要假定 T 具備 Copy：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_impl.rs:implementation"
```

`impl<T>` 的宣告不能省掉，只留下 `impl Holder<T>` 然後期待 T 自動存在。這與泛型函式先在名稱後宣告 T，再供參數與回傳型別使用，是同一項先宣告、再使用的安排。

呼叫 `Holder::new(120)` 時，編譯器從輸入與接收變數推導 T；`number.into_value()` 則交回 i32。方法的 self 已表明接手盒子，後面不再用原 number：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_impl.rs:number"
```

下面再用第 3.8 節已會建立的 String 驗證同一個 impl。label move 進 `Holder<String>`，接著由 into_value 交到 saved_label；泛型方法沒有另外複製文字。

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_impl.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example generic_impl --locked
```

```text
接回數字=120
接回文字=訂單A
```

## 一個型別參數，可以用於兩個欄位

**下面的 Pair 宣告一個型別參數 T，並讓兩個欄位都使用這個 T。** 因此同一筆 Pair 的 first 和 second 必須採用相同型別。

這份完整失敗例卻把 first 設為整數、second 設為 bool：

```rust
--8<-- "examples/intro-workspace/data-steps/fixtures/compile-fail/generic_same_type.rs"
```

放入獨立練習專案執行 `cargo check`，會得到 **E0308**。兩個欄位共用的一個 T 無法同時是 i32 與 bool。

若原需求規定同型別，應修正輸入；若本來就允許不同型別，就需要兩個型別參數。

## 練習與完整答案

定義 `Pair<T, U>`：first 使用 T，second 使用 U。再寫 `pair<T, U>` 接收這兩份值，回傳同樣型別安排的 Pair。逗號分開的是兩個型別參數；每個參數仍各自對應自己的欄位。

建立 `Pair<i32, bool>` 保存 120 與 true，以已知數值／布林 assertion 檢查，再分別顯示。完整答案沿用本頁 manifest，欄位同名簡寫仍與 new、hold 相同：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/generic_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example generic_answer --locked
```

輸出是 `金額=120，是否可用=true`。現在可以分清型別參數的數量、欄位的數量，以及同一參數在各位置保留的型別關係；下一節把這項語法用到標準的結果型別。

[下一節：Option 與 Result](05-02-lesson.md)
