# 3.1 struct：為訂單欄位命名

第 1.8 節把一筆訂單保存成 tuple，依位置約定哪一項是單價、哪一項是數量。看到 `order.0` 時，仍要記得這個約定。現在保留單價 120、數量 3，改讓資料直接帶著欄位名稱；計算仍是相乘，不增加另一種計價規則。

## 先定義資料形狀，再建立實例

**struct** 是把相關欄位組合起來的自訂型別。欄位各有名稱與型別，像這裡的 `unit_price: i32` 與 `count: i32`。`Order` 是型別名稱，通常以大寫開頭：

```rust
--8<-- "examples/intro-workspace/ownership/examples/structs.rs:fields"
```

這段定義「一筆 Order 有什麼」，還不是一筆具體訂單。要建立**實例**，寫出型別名稱，再於大括號內逐一提供欄位值：

```rust
--8<-- "examples/intro-workspace/ownership/examples/structs.rs:instance"
```

每項的冒號左側是欄位名，右側是這筆資料的值；逗號分隔欄位。即使把兩個欄位的列出順序交換，只要仍提供對的名稱，單價與數量的角色就不會交換。這與 tuple 按位置對應的方式不同。

`order.unit_price` 讀取單價，`order.count` 讀取數量。點號後現在是欄位名稱，而不是 tuple 的位置號碼。

## 修改一個欄位，其他材料保留

`let mut order` 允許經由這個 binding 修改資料；`order.count += 1;` 只將數量由 3 加到 4，單價仍是 120。`mut` 放在持有實例的變數宣告，不是放在 struct 的欄位定義：

```rust
--8<-- "examples/intro-workspace/ownership/examples/structs.rs:update"
```

先算出的 `before` 是 360，修改數量後再算 `after` 才得到 480。先前保存的數字不會自行重新計算，與第一章的變數規則相同。

## 完整範例與執行

本章使用 `ownership/` 成員，package 名稱是 `intro-ownership`。它沒有 `src/lib.rs`；每個例子都有自己的完整定義與 `main`，不會載入其他節的資料模型。完整設定如下：

```toml
--8<-- "examples/intro-workspace/ownership/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/ownership/examples/structs.rs"
```

從教材 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example structs --locked
```

```text
原小計=360，加一件後=480
```

struct 改變的是資料的表示：相關值保存在一筆資料裡，欄位名稱說明角色。tuple 仍能完成原來的乘法；不需要為了計算正確才改成 struct。

## 跨 module 使用：型別與欄位各自公開

第 2.6 節用 `pub` 公開函式。若資料模型也放到另一個 module，外面的程式要建立它，就需要能使用型別和所需欄位。`pub struct Order` 公開型別，欄位前各自的 `pub` 公開該欄位；只公開型別，不會自動公開所有欄位。

這份完整模型檔位於 `examples/public_model/model/mod.rs`，仍只有相同的兩個數字欄位：

```rust
--8<-- "examples/intro-workspace/ownership/examples/public_model/model/mod.rs"
```

範例根檔案的 `mod model;` 載入它，`use crate::model::Order;` 從這個範例自己的根找到公開型別。兩項規則都沿用第 2.6 節。

下面顯示欄位時，還會用到 `println!("數量={}", order.count)` 這種格式：空的 `{}` 按順序對應後面逗號分隔的 expression。它能直接顯示欄位或計算結果，不必先替每個結果建立變數；`{名稱}` 則仍可沿用。

這是呼叫模型的完整 `examples/public_model/main.rs`，設定沿用本頁 manifest，沒有其他 helper：

```rust
--8<-- "examples/intro-workspace/ownership/examples/public_model/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example public_model --locked
```

輸出是 `跨 module 的數量=3，小計=240`。型別與欄位前的公開標記，讓範例根能在 module 外建立、修改和讀取它；我們仍只顯示已知的數字欄位，沒有直接顯示整筆 Order。

## 練習與完整答案

建立單價 80、數量 2 的 Order。先保存小計，再把同一筆的數量加一，算出新小計；用第 2.7 節的數值 assertion 確認 160、240。先想清楚哪個變數需要 `mut`，再寫欄位更新，不要同時改掉單價。

完整答案沿用本頁 manifest，所有定義都在同一份檔案：

```rust
--8<-- "examples/intro-workspace/ownership/examples/structs_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-ownership --example structs_answer --locked
```

兩項檢查成功後，會顯示 `原小計=160，加一件後=240`。現在已會把整筆資料交給一個變數；下一節要問，把它再指派給另一個變數後，原名稱還能不能用。

[下一節：ownership、move 與 Copy 行為](03-02-lesson.md)
