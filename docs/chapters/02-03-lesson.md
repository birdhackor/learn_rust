# 2.3 用 for 與 range 重複處理材料

價目表 `[120, 80, 200]` 的總和是 400。現在可以逐項索引再相加，但每多一筆就得再寫一次操作。我們想保留「每項都加進總額」這個規則，讓程式自行走完材料。

## for 每次取得一項值

**迴圈**讓同一段工作執行數次。`for price in prices { ... }` 會依 array 順序取出每項，這次把該值命名為 `price`，執行一次大括號內的 body。這裡走訪的是整數 array，因此每次得到一個 `i32` 價格，並非索引。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/for_range.rs:array-loop"
```

先在迴圈外建立 mutable 的 `total`，讓各次處理更新同一個累積值：

| 這次的 price | 原本 total | 加完的 total |
| --- | --- | --- |
| 120 | 0 | 120 |
| 80 | 120 | 200 |
| 200 | 200 | 400 |

`price` 是當次 body 裡的名稱；`total` 則來自外層，離開迴圈後仍能顯示最後的 400。如果把 `let mut total = 0;` 放進 body，每次都會重新從零開始，就無法保存前一次加總。

這裡不必自己更新索引或另寫「一共三項」的結束條件。array 的每項都處理一次，材料用完就結束。改成四項價目表時，for 的走訪寫法仍相同；固定 array 的型別與資料則要一起改為四項。

## range 提供一段連續整數

有時候要處理的材料是 0、1、2 這樣連續的數字，不需要先寫一個 array。**range** 表達一段範圍；把它放在 for 的 `in` 後，便能依序取得範圍中的整數。

| 範圍 | 包含哪些整數 |
| --- | --- |
| `0..3` | 0、1、2；包含起點，不含終點 |
| `0..=3` | 0、1、2、3；兩端都包含 |
| `3..3` | 沒有任何項目 |

`..` 與 `..=` 是範圍寫法，不是小數點，也不表示先建立一個存放全部數字的 array。這裡的整數範圍朝較大的值前進；對不含終點的 `start..end`，起點等於或大於終點時，就沒有項目。

下面把起終點當作函式參數，累積不含終點的範圍：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/for_range.rs:range-function"
```

`range_sum(0, 3)` 加上 0、1、2 得 3；`range_sum(3, 3)` 一次 body 都不執行，回傳原本的 0。這是「沒有材料就不做」的情況。

若要連 3 一起加進去，改用包含終點的範圍：

```rust
--8<-- "examples/intro-workspace/control-steps/examples/for_range.rs:inclusive-loop"
```

這次總和是 0+1+2+3，也就是 6。選擇是否包含終點，取決於你希望處理哪些數字；不要為了湊答案才隨意加一。

## 完整範例與執行

```toml
--8<-- "examples/intro-workspace/control-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/control-steps/examples/for_range.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example for_range --locked
```

```text
價目總和=400
不含終點的總和=3
包含終點的總和=6
起終點相等的半開範圍總和=0
```

這裡「半開範圍」只是指包含起點、排除終點的 `..` 範圍。總和不同，來自是否真的多處理了最後一個 3。

## 練習與完整答案

定義 `sum_three(values: [i32; 3]) -> i32`，用 for 加總後回傳結果。參數型別 `[i32; 3]` 已在第一章學過，現在把整個三項 array 提供給函式；本題只接受三項，不需要其他資料型別。

用 `[120, 80, 200]` 驗證 400，再用 `[0, -5, 5]` 驗證 0。第二組材料確認函式確實加上每個輸入，而不是把 400 寫死。

```rust
--8<-- "examples/intro-workspace/control-steps/examples/for_range_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-control-steps --example for_range_answer --locked
```

完整答案沿用本頁設定，顯示 `第一組=400，第二組=0`。已有一列材料或一段整數範圍時，for 直接表達每項各處理一次。下一節則改由持續變動的條件判斷還要不要做下一次。

[下一節：while 的條件與狀態更新](02-04-lesson.md)
