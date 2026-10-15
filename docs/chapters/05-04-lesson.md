# 5.4 Vec：新增、查詢及移除清單元素

array 的項數寫在型別裡，slice 借用已有的一段資料。現在有一份價格清單，處理過程要新增末尾價格、移除其中一項，再取回最後一項；長度會隨工作變動。這裡使用標準函式庫的 **`Vec<T>`**，也稱 vector。

## Vec 持有同型別元素，長度可以改變

`Vec<T>` 擁有一列依位置排列、元素型別同為 T 的資料。`Vec<i32>` 保存數字，`Vec<String>` 保存文字；同一份 `Vec<i32>` 不能混入 bool。與 array 不同，長度不寫在 Vec 的型別裡。

對本節的 i32 元素，存放區在 heap 且連續，由 Vec 管理。**Vec::new()** 建立空 Vec，此刻不預先配置元素空間；可從接收變數的 `Vec<i32>` 註記，或後續加入的值，推導 T。Vec 已由 prelude 提供，不需另外下載或 use。

**push(value)** 需要修改借用，在末尾加入一個元素，回傳 unit。它以值接收元素，i32 會 Copy；非 Copy 元素的交接留到下一節觀察。**len()** 只需 shared 借用，回傳目前的項數 usize，這裡數的是元素，不是文字 bytes。

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs:build"
```

新增三次後，順序為80、120、200，長度是3。values宣告成mut，才能供push借改。

## as_slice 與 get：借讀其中一項

**as_slice()** shared 借用 Vec 的整段內容，回傳 &[i32]，讓第3.7節的slice操作可以繼續使用；它不交出Vec的ownership，也不複製元素。

slice 的 **get(position)** 是按元素索引查詢，這次參數是usize位置，回傳 **`Option<&i32>`**。索引小於長度時，Some保存元素的shared reference；否則None。它不移除元素，也不交出owned i32。

因此 `values.as_slice().get(1)` 先借整段，再查索引1；Some(value)中的value是&i32，以*value讀出Copy數字：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs:get"
```

索引1取得120；索引9沒有對應元素。這與上一節str.get(range)的參數和索引單位不同：現在選一個元素，不是byte區段。

已確認索引有效時，也可用 `values.as_slice()[0]` 讀i32；它會Copy數字，仍有第3.7節的越界panic檢查。

## remove：取走指定項，後面向前補

**remove(position)** 需要修改借用，移除該索引的元素並交回owned T；此刻T是i32，所以交回i32。position必須小於當下長度，否則panic，不以None表示失敗。

移除後，後面元素向前補，保留原相對順序。從80、120、200移除索引0，交回80，留下120、200；新的索引0因此是120：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs:remove"
```

相同索引在移除前後可以指向不同項，不能把索引當成永久身分。

## pop：取回末尾，空清單回傳 None

**pop()** 同樣需要修改借用，移除最後一項，回傳 `Option<T>`。有元素時Some交回owned值；空清單時None，不因沒有末尾而panic。

現在剩下120、200，這次pop取得Some(200)，清單留下120：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs:pop"
```

get借讀元素，pop交出末尾元素，remove則交出指定位置的元素；它們的回傳型別與失敗契約要分別閱讀。

## as_mut_slice：借改既有元素

**as_mut_slice()** 獨占借用整段內容，回傳&mut [i32]。可以修改這些既有元素，不能透過這份slice新增項目或改變長度。

下面writable借到只剩120的清單，把第0項加10。writable在這次指派後不再使用，所以後面的println才能回到Vec借讀，沿用第3.5節的最後使用規則：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs:modify"
```

結果是130，修改的是Vec原本持有的元素。

## vec!：從已列出的值建立清單

**vec![...]** 是標準的macro，直接以逗號分隔的同型別值建立Vec。`!`仍是前文已教的macro呼叫標記；中括號裡是初始元素，不是固定長度array型別。

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs:macro"
```

這份initial有三項，也能之後push或pop；`vec![120,80,200]`與array值`[120,80,200]`保存相同數字，但容器型別與長度規則不同。

## 完整程式與執行

```toml
--8<-- "examples/intro-workspace/data-steps/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_operations.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example vector_operations --locked
```

```text
新增後的長度=3
索引1的數字=120
沒有索引9
移除索引0的數字=80
剩下的第一項=120
取回最後一項=200
修改剩下的第一項=130
vec! 建立的長度=3
```

### remove 越界會 panic

以下是完整的預期執行失敗例。position等於長度，已是最後一項之後：

```rust
--8<-- "examples/intro-workspace/data-steps/fixtures/runtime-fail/vector_remove_bounds.rs"
```

放入獨立練習專案執行，錯誤包含`removal index`。知道Option，並不表示每個Vec方法都以Option保存失敗；remove與pop各有自己的契約。

## 練習與完整答案

用vec!建立10、20，再push30。remove索引0應交回10；把留下的第一項經由as_mut_slice加5，應變25。接著連續pop三次，依序取得30、25、None，最後長度為0。

本題直接讀索引0，是因為移除後已知還有兩項。`for _ in 0..3`重複三次pop；range取得的數字不需使用，因此以第4章的_ pattern略過。其餘處理只用本頁已教的方法：

```rust
--8<-- "examples/intro-workspace/data-steps/examples/vector_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-data-steps --example vector_answer --locked
```

```text
移除第一項=10
留下的第一項=25
取回末尾=30
取回末尾=25
清單已空
剩餘項目=0
```

這份答案用usize的相等條件交給assert!，仍只要求已教的bool檢查。下一節再用非Copy的String看清單中的值由誰持有，最後組成完整整理函式。

[下一節：Vec的ownership與清單整理](05-05-lesson.md)
