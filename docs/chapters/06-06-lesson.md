# 6.6 用 HashMap 分組，讓結果擁有自己的鍵

這次輸入依序是 Rust、Python、Rust，要得到 Rust出現2次、Python出現1次。上一節的 insert 會取代值，不會自動累加。現在用 get_mut 分辨已有名稱與新名稱，讓同名標籤累積到同一筆結果。

## 每次標籤進來，先問鍵是否已存在

函式只借讀 `&[String]`，交回新的 `HashMap<String, usize>`。沿用第 3.7 節走訪 borrowed slice 的方式，這次元素換成 String：`for label in labels` 每次取得一份 **`&String`**，沒有把 owned 文字移出輸入。

`label.as_str()` 提供查詢視圖。找到時，get_mut 交出 `&mut usize`，經由 `*count += 1` 更新原值；缺鍵時，才需要一份可交進 map 的 owned String。

**`(*label).clone()`** 先沿 reference 找到原 String，再呼叫它的 Clone 借讀並建立獨立文字。這個方法呼叫沒有把原 String move 出 slice。它只出現在缺鍵分支，因為已有鍵時不需要再擁有另一份同名文字。

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/grouping_support/mod.rs:count-labels"
```

Some 分支已取得修改 reference，直接改它借到的值。None 分支沒有取得要繼續使用的值 reference，查詢借用可以結束，再用 insert 加入新鍵。

對這次材料，狀態依序是：

| 讀到的標籤 | 查詢與本次工作 | 處理後保存的次數 |
| --- | --- | --- |
| Rust | None；複製名稱，加入1 | Rust→1 |
| Python | None；複製名稱，加入1 | Rust→1、Python→1 |
| Rust | Some；修改已有次數 | Rust→2、Python→1 |

第三次沒有把 Rust 改回1，也沒有新增另一筆相等鍵。重複名稱正是用得到「先查詢，再依情況工作」的材料。

## 呼叫者保留輸入，結果持有獨立名稱

本節沿用上一節的 owned_text helper 建立輸入；其完整內容仍只有建立 String 的工作：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/map_support/mod.rs"
```

呼叫者以 `labels.as_slice()` 借讀輸入，接回 map 後，依固定的三個查詢名稱顯示結果。這份 `&str` array 的 for 每次取得一個 &str；查不到的名稱顯示0（未收錄），只是報表選定的顯示方式，沒有新增鍵。

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/grouping.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example grouping --locked
```

```text
Rust=2
Python=1
JavaScript=0（未收錄）
原清單仍有3個標籤
```

輸入保留三份標籤；map只有兩個不同名稱。map的鍵是缺鍵時複製出的 String，不是借用輸入的 &str，因此輸入結束後仍能查詢結果。

## 用測試檢查重複、空輸入與 owned 結果

測試直接比較 get 的 `Option<&usize>`，已由第 6.2 節教過其 Debug 與 PartialEq：None與Some不同，Some中的reference比較借到的數字。這裡不需要用長度或假數字代替完整查詢結果。

最後一項測試特別把輸入放進內層 block，再把 count_labels 回傳的 owned map 交給外層 counts。這沿用第 1.6 節的尾端 expression；離開內層後，labels與其文字結束，map自己的鍵仍保留。

下面是**完整** `grouping_support/mod.rs`，包括函式與全部三項測試：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/grouping_support/mod.rs"
```

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example grouping --locked
```

三項分別檢查同名累加、空清單沒有鍵，以及輸入scope結束後結果仍可使用。這個 target 載入的模組只有上述 map_support 與 grouping_support，不會隱含載入其他章的 library。

## 自訂鍵：兩個欄位一起決定身分

庫存有時同時依地區與類別分組，不能只看其中一個數字。用 GroupKey 保存兩個 i32，讓全部欄位一起決定是否為同一組。

第 6.2 節的 derived PartialEq 逐欄比較；**derived Eq** 要求各欄位都有 Eq，才能保證每個組合與自己相等。**derived Hash** 則使用各欄位的 Hash 資訊，欄位也必須有 Hash。

本例的 i32 都符合。一起產生 PartialEq、Eq、Hash，就以同一套欄位資料決定相等與hash。若自行修改其中一項，例如比較時忽略類別，便要重新核對相等鍵hash一致，不能任意混搭。

這次查詢借用完整的同型別鍵，傳入 `&same`。下面分別只改類別、只改地區，讓兩個欄位都參與鍵身分的效果可見：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/custom_keys.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example custom_keys --locked
```

```text
地區1、類別2=3
地區1、類別3未收錄
地區2、類別2未收錄
```

只有地區1、類別2有值3。若少了Hash，即使型別能相等比較，也不能完成HashMap所需的鍵查找。下面是完整預期編譯失敗例：

```rust
--8<-- "examples/intro-workspace/trait-stages/fixtures/compile-fail/key_without_hash.rs"
```

編譯器以 **E0599** 指出insert的trait條件未滿足，其中缺少GroupKey的Hash。這和第6.1節缺少bound一樣，都是使用某項操作時沒有提供所需能力。

## 完整設定與練習答案

本節所有成功targets仍使用同一份完整member設定：

```toml
--8<-- "examples/intro-workspace/trait-stages/Cargo.toml"
```

前面已列grouping的根與兩個完整helper，以及自訂鍵的完整程式。接著將「每個標籤加一」改為「每筆庫存加上數量」。

建立 `StockItem`，包含 `name: String` 與 `quantity: usize`。函式 `quantity_by_name(items: &[StockItem]) -> HashMap<String, usize>` 只借讀輸入，回傳擁有名稱的map。for每次取得`&StockItem`；讀quantity可Copy數字，name則以`as_str()`查詢，缺鍵時才用String的clone建立owned鍵。

材料依序為Rust2件、Python3件、Rust6件。先預測結果：Rust8、Python3、缺JavaScript，map有兩個名稱，原輸入仍有三筆。若每筆都直接insert新數量，Rust會只剩6，不能完成合計。

下面是完整答案，沿用本頁的manifest與完整map_support，不修改count_labels：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/grouping_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example grouping_answer --locked
```

```text
Rust 總量=Some(8)
Python 總量=Some(3)
原清單仍有3筆資料
```

共同的查詢／缺鍵／更新結構保留，累加材料由固定的1換成本筆quantity。到這裡，trait能力、標準比較與複製契約都用在了完整資料整理工作；下一章再描述函式交回reference時的來源關係。

[下一章：lifetime annotation與來源](07-01-lesson.md)
