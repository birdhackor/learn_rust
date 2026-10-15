# 1.8 用 tuple 與 array 保存多個值

一筆訂單同時有單價、數量、是否備妥；一張價目表則有三個同型別的價格。我們需要把相關資料放在一起，但這兩種材料的安排不同：訂單的每個位置各有意義，價目表的每個位置則保存同一類資料。

## tuple：每個位置各有型別

**tuple**（元組）把固定數量的值合成一個值。以 `(120, 3, true)` 表示這次選定的訂單順序：第一格單價、第二格數量、第三格是否備妥。逗號分隔各格，外面用小括號。

型別 `(i32, i32, bool)` 同樣逐格對應：前兩格為整數，第三格為布林值。取出個別位置時，`order.0` 代表第一格；位置從 0 開始，接著是 `.1`、`.2`。也可以用 `let (名稱一, 名稱二, 名稱三) = order;` 按順序替各格命名，這叫作**解構**。

下面先保存整個 tuple，再以這兩種方式取得材料：

```rust
--8<-- "examples/intro-workspace/steps/examples/tuple_array.rs:tuple"
```

`order.0` 取得第一格的 120。這裡的點與數字是取出固定位置的寫法，不是小數點。

`let (unit_price, count, ready) = order;` 則一次按順序替三格命名。三個名稱依序得到 120、3、true。左右的位置數要對得上；tuple 建立後，這個型別的格數不會增減，每格的型別也固定。

這裡的格子名稱由我們約定；`order.0` 本身不會告訴讀者「這是單價」，所以先解構成有意義的名稱會更容易讀。第 3 章會再學替資料欄位正式命名的做法。

## array：同型別、固定項數

價目表的三項都是 `i32`。**array**（陣列）保存相同型別、固定項數的元素；元素就是容器裡各自的一個值。用中括號寫出元素：`[120, 80, 200]`。

型別寫成 `[i32; 3]`，表示「三個 `i32`」。這裡的分號分開元素型別與項數，與前面結束工作那種行尾分號的用途不同。選位置時，用 `prices[索引]`；索引是從 0 開始的號碼，所以三項的合法索引為 0、1、2。以下先選出第一項、第三項：

```rust
--8<-- "examples/intro-workspace/steps/examples/tuple_array.rs:array"
```

`prices[0]` 這次得到 120，`prices[2]` 是 200。索引是選哪一項的號碼，與那一項保存的價格不同。

`prices.len()` 是**方法呼叫**：在一個值後面用點接操作名稱，再加括號。這個方法回傳元素項數，本例是 3；空括號表示這次不必再提供其他輸入。現在先會使用 array 的 `len` 即可，後面才學如何為自己的資料定義方法。

索引和 `len()` 的結果使用 `usize`，一種不帶正負號的整數型別，表示空間的大小依目標電腦而定，常見 64 位元目標上為 64 bits。`let index: usize = 1;` 讓索引保存在變數中；`prices[index]` 因而得到第二項 80。不同於 tuple 的 `.0`，array 的索引可以來自執行時才算出的變數。

現在把索引放進變數，再顯示項數與選出的值：

```rust
--8<-- "examples/intro-workspace/steps/examples/tuple_array.rs:selection"
```

輸出為 `項數=3，選到=80`。

`[i32; 3]` 與 `[i32; 4]` 是不同型別；同一個 array 不能靠多加一項變成四項。若將來要保存會增減的清單，需要另一種資料型別，第 5 章再教。

## 固定項數仍然可以修改元素

項數不變與元素值不變是兩個問題。若三筆訂單數量起初都是 0，可以寫 `[0; 3]`，表示重複三次相同的數值；它等同 `[0, 0, 0]`。這是建立值的寫法，而 `[i32; 3]` 是描述型別的寫法，請先看中括號左半部是值還是型別。

因為 `counts` 有 `mut`，`counts[1] = 2;` 可以把第二個元素改成 2；長度仍是 3。少了 `mut`，就不能經由這個 binding 修改元素。

```rust
--8<-- "examples/intro-workspace/steps/examples/tuple_array.rs:repeat"
```

## 完整範例與執行

```rust
--8<-- "examples/intro-workspace/steps/examples/tuple_array.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example tuple_array --locked
```

```text
單價=120
拆開後：單價=120，數量=3，備妥=true
第一項=120，第三項=200
項數=3，選到=80
中間數量=2
```

## 索引超出範圍會怎樣

長度為 3，代表有三項；索引 3 卻是在要求**第四項**。Rust 會檢查索引是否小於長度。若不符合，這次索引操作會 **panic**：停止正常執行並報告錯誤。在本節這種單一主程式中，程式會以失敗狀態結束，後面的顯示工作不會繼續。

下面是完整的預期執行失敗程式。`selected_price` 是我們在這裡定義的函式：收一個 `usize` 索引，回傳所選的 `i32`。`main` 刻意傳入 3，讓索引在函式執行時超出範圍：

```rust
--8<-- "examples/intro-workspace/steps/fixtures/runtime-fail/array_index.rs"
```

若想重現，可以把這份完整內容放進第 1.2 節自建專案的 `src/main.rs`，保留當時的 manifest，再執行 `cargo run`。錯誤重點會包含：

```text
index out of bounds: the len is 3 but the index is 3
```

程式不會印出 `價格=...`。與第 1.6 節的名稱找不到、回傳型別不符相比，這份程式的型別與名稱都合法，卻在執行時因這次索引失敗。編譯器若能直接判斷某次索引必定超界，也可能提早拒絕它；不能以「能編譯」推定每次索引都有效。

這個檢查避免程式把 array 範圍外的資料當成元素。現在先學會辨認邊界：長度 3，最後合法索引 2。第 2 章會用條件判斷，在選取之前決定應該怎麼做。

## 練習與完整答案

建立 `(80, 2, true)` 的訂單 tuple，解構後計算商品總額。再建立三項價目表 `[120, 80, 200]`，用 `usize` 變數選索引 2，顯示選到的價格與項數。預期總額是 160，價格是 200，項數是 3。

```rust
--8<-- "examples/intro-workspace/steps/examples/tuple_array_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-steps --example tuple_array_answer --locked
```

兩種容器都把多個值合在一起：tuple 讓各位置有自己的型別，array 則以相同元素型別與固定項數組成。接下來要讓程式依材料選擇不同工作，而不是每次都照同樣順序計算。

[下一章：流程控制](02-01-lesson.md)
