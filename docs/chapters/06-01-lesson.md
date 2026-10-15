# 6.1 用 trait、impl 與 bound 要求共同能力

第 3.6 節的訂單可以由單價與數量算出金額。現在還有一種組合商品，只保存整組價格。兩者的資料形狀不同，接下來卻都要回答「金額是否至少 300」。我們希望這項門檻規則保存在一處，而每種資料仍依自己的欄位取得金額。

本節沿用非負的小整數材料，乘法不超過 `i32` 範圍；不追加運費或缺少報價的政策。先讓兩種資料提供共同操作，再用第 5.1 節的泛型呼叫它。

## 先保留各自的資料形狀

`Order` 保存單價 `unit_price` 與數量 `count`；`Bundle` 保存整組商品的 `price`。訂單金額要相乘，組合金額則直接讀取已保存的價格。

本章的階段範例位於 `examples/intro-workspace/trait-stages/`，package 名稱為 `intro-trait-stages`。它沒有 library，各範例只載入自己的模組。這一節的 `charge_support` module 位於 `examples/charge_support/mod.rs`，先定義公開的型別與欄位，讓呼叫者可以建立資料：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charge_support/mod.rs:charge-model"
```

它們仍是兩個型別。共同的工作不要求 `Bundle` 也保存單價和數量。

## trait 描述使用者需要的操作

**trait** 描述型別應提供的能力。本例的 **`Charge`** 只要求一個 `amount` 方法：借讀這份資料，交回一個 `i32` 金額。

`trait Charge { ... }` 宣告這項契約；`pub` 讓其他 module 可以使用。裡面的 `fn amount(&self) -> i32;` 是方法簽名，`&self` 依第 3.6 節代表 shared 借讀，實作於 `Order` 時借讀 `Order`、實作於 `Bundle` 時借讀 `Bundle`。現在不寫 body，而在簽名後加分號，留給各型別提供做法：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charge_support/mod.rs:charge-trait"
```

宣告 trait 沒有自動替每個型別加上 `amount`。它先把使用者所需的輸入權限與輸出型別說清楚；使用者不用知道金額由哪些欄位算出。

## 各型別實作自己的取金額方式

第 3.6 節的 `impl Order` 為型別定義自己的操作。現在的 **`impl Charge for Order`** 額外說明：這份實作遵守 `Charge`。方法簽名要符合 trait，body 則依 `Order` 的欄位相乘。`Bundle` 的實作以自己的 `price` 提供相同意義的結果：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charge_support/mod.rs:charge-implementations"
```

trait 方法依公開契約使用，這裡不在實作的 `fn` 前再加 `pub`。兩份 body 可以不同；我們統一的是「取得金額」的能力，沒有把資料合成同一型別。

## bound 讓泛型 body 有理由呼叫方法

第 5.1 節的泛型函式只保存、交回 `T`，不能假定每個 `T` 都能相乘或顯示。現在要在泛型 body 裡呼叫 `amount`，必須先寫出這項要求。

**trait bound** 是對型別參數的能力要求。`T: Charge` 表示 `T` 必須實作 `Charge`，因此編譯器允許 body 依這項契約呼叫 `value.amount()`。

`meets_threshold` 接受 `&T`，只借讀資料；另一個參數是門檻，結果則是 `bool`。取得金額後用已教的 `>=` 比較：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charge_support/mod.rs:charge-bound"
```

這裡的泛型 reference 型別 `&T`，仍依第 3 章的 borrowing 規則使用。兩次呼叫共用同一份門檻判斷，各自選用所傳型別的實作：

| 這次資料 | `amount` 如何取得金額 | 門檻 300 的結果 |
| --- | --- | --- |
| `Order`：單價 120、數量 3 | 相乘得到 360 | true |
| `Bundle`：價格 160 | 直接交回 160 | false |

原本也能各寫一個判斷函式；這次讓共同比較規則只有一個位置，各資料的取金額方式留在各自實作。

## module 外的呼叫者也要引入 trait

`mod charge_support;` 沿用第 2.6 節的規則，從同層的 `charge_support/mod.rs` 載入模組；該資料夾沒有 `main.rs`，不會成為另一個 Cargo 範例。

透過點號呼叫 trait 方法時，提供該方法的 trait 也要在目前 scope 中。所以下面不只引入資料型別與函式，還有 `use crate::charge_support::Charge;`。這讓編譯器找到方法；它沒有取得資料 ownership，也沒有下載依賴。

這個 example 自己就是一個 crate，`crate::` 從這個範例的根開始：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charges.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example charges --locked
```

```text
訂單小計=360，組合價=160
訂單達300=true
組合達300=false
訂單仍可讀，數量=3
```

最後仍可讀 `order.count`，因為 `amount` 與門檻函式都只借讀。trait 沒有改掉原有的 ownership 規則。

## 移除 bound，會失去哪項保證

下面是完整的預期編譯失敗程式：保留 trait 定義，卻從函式的 `T` 移除 `: Charge`。

```rust
--8<-- "examples/intro-workspace/trait-stages/fixtures/compile-fail/missing_bound.rs"
```

放進第 1.2 節的獨立專案執行 `cargo check`，會得到 **E0599**：泛型 `T` 沒有可用的 `amount`。錯誤發生在檢查泛型定義時，不必等某筆訂單執行才知道。bound 表達 body 實際需要的能力，不是替計算結果作擔保。

## 完整來源與數值測試

完整 member manifest 仍只有三個已教欄位：

```toml
--8<-- "examples/intro-workspace/trait-stages/Cargo.toml"
```

前面已列完整 `charges.rs`；下面是它實際載入的完整 `charge_support/mod.rs`。其中兩項測試沿用第 2.7 節的 `cfg(test)`、`test` 與 assertion，只比較數字與 bool：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charge_support/mod.rs"
```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example charges --locked
```

兩項測試核對兩種金額及門檻邊界：300 達到 300、卻沒有達到 301。trait 保證方法與型別符合要求，計價是否符合商業規則仍由實作與測試核對。

## 練習與完整答案

新增 `DiscountedBundle`，保存 `original_price: i32` 與 `discount: i32`。材料採非負小整數，折扣不超過原價；實作 `Charge`，讓金額為原價減折扣。使用同一份 `meets_threshold`，不要再複製一次比較。

以原價 360、折扣 80，確認金額 280，300 門檻為 false、280 門檻為 true。下面是完整答案；它載入的是同頁完整的 `charge_support/mod.rs`，manifest 也沿用本頁：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/charges_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example charges_answer --locked
```

輸出 `折扣後=280，達300=false`。答案也以原有 Order、Bundle 核對同樣金額，三種資料都使用 `Charge` 的操作。加入新的實作者之後，共同門檻函式保持原來的工作。下一節用標準 trait 讓整筆資料可以顯示與比較。

[下一節：Debug、PartialEq 與 derive](06-02-lesson.md)
