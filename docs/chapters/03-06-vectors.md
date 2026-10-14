# 3.6 用 Vec 整理長度可變的清單

array 的長度寫在型別中，slice 則借用已有的一段資料。現在要把數列 `[120, -5, 0, 80]` 中的非負數保留下來，仍按原順序輸出。答案可能有三項、四項或零項，因此需要一個能隨處理結果增加項目的容器。

## Vec 擁有一列同型別資料

**`Vec<T>`** 是標準函式庫提供的清單型別，也稱 vector。它擁有一列依位置排列、元素型別同為 T 的資料，長度可以在執行時改變。對本節的 i32／String，配置的元素空間位於 heap 且連續；`Vec::new()` 先建立空容器，不預先配置元素空間。`Vec<i32>` 保存數字，`Vec<String>` 保存 owned 文字，不能在同一份 `Vec<i32>` 混入 bool。

和 array 相比，Vec 的型別沒有寫元素數量；和 &[i32] 相比，Vec 管理資料本身，slice 只是借用既有內容。Vec 結束時，也結束它持有的各項值，依元素型別清理需要的資源。

先把本節要用的操作與契約列清楚：

| 操作 | 需要與結果 | 邊界與用途 |
| --- | --- | --- |
| `Vec::new()` | 建立空 Vec | 用型別註記或後續 push 的值提供 T 的推導線索。 |
| `vec![120, 80, 200]` | 用已列出的同型別值建立 Vec | 這個 macro 的逗號清單提供初始項目；不會建立固定長度 array。 |
| `values.push(value)` | 借用以修改 Vec，將值加到末尾，回傳 unit | 接收元素的 ownership；i32 會 Copy，String 則 move。 |
| `values.len()` | shared 借用，回傳目前項目數，型別 usize | 這裡數元素，不是文字 byte 數。 |
| `values.pop()` | mutable 借用，移除末尾並回傳 `Option<T>` | 有項目則 Some 中交回 owned 值；空清單為 None。 |
| `values.remove(position)` | mutable 借用，移除該索引並交回 T | 索引須小於長度，否則 panic；後面的元素向前補，順序保持。 |
| `values.as_slice()` | shared 借用整份 Vec，回傳 &[T] | 讓已有的 slice 函式處理資料，不接手清單。 |
| `values.as_mut_slice()` | mutable 借用整份 Vec，回傳 &mut [T] | 修改既有元素，使用期間不能衝突地存取原 Vec。 |

這裡的 slice **`get(position)`** 是另一個新的安全取得接口：位置合法時回傳 `Option<&T>` 的 Some，不合法時是 None。它借用原元素，不移除元素，也不交回一份 owned T。先用 as_slice 取得視圖，再用 get，才能清楚對應「擁有清單」與「借用其中元素」的兩個步驟。

在 numeric slice 上直接用 `[position]` 取 i32，會 Copy 那個數字；索引仍必須有效，和 2.6 的邊界規則相同。接收到可能越界的位置時，用 get 並處理 None；自己依長度走訪所有位置時，可用已確認有效的索引。

```rust
--8<-- "examples/ch03-data/examples/vector_operations.rs"
```

```bash
cargo run -p ch03-data --example vector_operations --locked
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

原清單是 80、120、200。移除索引 0 之後，120 變成新的索引 0；pop 再交回 200，剩下的只有 120。as_mut_slice 借用這個既有元素，把它加 10；借用最後使用後，回到 owner 讀到 130。不要把 remove 前後的相同索引當成永遠代表同一項。

### pop 與 remove 的失敗契約不同

pop 接收的是「取末尾」這個請求，空清單會交回 None；remove 接收明確索引，它不會以 None 表示越界。下面的索引等於長度，因此沒有對應元素，故意觸發 panic：

```rust
--8<-- "examples/ch03-data/fixtures/runtime-fail/vector_remove_bounds.rs"
```

錯誤包含 `removal index`。知道 Option 並不表示所有清單操作都自動採用 Option；每個接口的契約仍要分別閱讀。

## 清單中的值仍有 ownership

把 String push 進 `Vec<String>`，會把文字的管理責任交進清單；原變數不能再使用這份 String。pop 則把最後一個 String 交給接收者，Vec 不再持有它。這和取得 `&String` 的 get 是不同工作。

```rust
--8<-- "examples/ch03-data/examples/vector_ownership.rs"
```

```bash
cargo run -p ch03-data --example vector_ownership --locked
```

```text
從清單取回=Rust
清單剩下的項目=0
```

message 的 ownership 先交進 messages，再由 pop 交到 taken。程式顯示的是 taken 借出的 &str；沒有從舊 message 名稱讀取，也沒有另外複製整份文字。整個 Vec 自身也沒有 Copy 能力，即使元素全是 i32，指派整個 Vec 仍會 move 它。

### 元素 reference 仍在使用時，不能增長清單

Vec 在 push 時若原有空間不足，可能請求更大的存放區並把元素交到新區域。原先借出的元素 reference 不能一邊繼續使用，一邊讓這種修改發生。

下面先用 get 借用第一項，push 後還要使用它，因此借用區間重疊，會收到 `E0502`：

```rust
--8<-- "examples/ch03-data/fixtures/compile-fail/vector_borrow_overlap.rs"
```

Option 中保存的也是 reference，沒有因為多包了一層 Some 就避開 borrowing。先完成原借用的最後使用，再修改 Vec，才是兩項依序的工作；不能依賴某次 push 恰好沒有更換存放區。

## 第一個清單成果：保留原順序的非負數

現在把資料表示與控制流程組起來。輸入是借來的 `&[i32]`，輸出是新的 `Vec<i32>`：原材料保留，結果的長度由實際保留多少項決定。規則是值至少為零就保留，負數略過，重複值仍保留原次數。

使用前文已教的 while：position 從 0 開始，每次先檢查它小於輸入長度，再索引該項；符合條件就 push，最後 position 加一。輸入沒有被修改，push 修改的是另一份結果清單。

這個函式放在 library 的 list module，pub／module／use 分工沿用 1.9：

```rust
--8<-- "examples/ch03-data/src/list.rs:list-rule"
```

數字被依原順序讀取，所以 push 的順序也是保留後的順序。對 120、−5、0、80，逐步得到一項 120，略過 −5，再接上 0、80。不是排序，也不是移除原資料中的負數。

呼叫程式借用 array，把新的 Vec 按已教的數字 range 索引顯示；先 as_slice，再索引它的元素：

```rust
--8<-- "examples/ch03-data/examples/list_demo.rs"
```

```bash
cargo run -p ch03-data --example list_demo --locked
```

```text
保留的項目=3
索引0=120
索引1=0
索引2=80
```

結果不是固定的三項：空輸入的 while 一次也不做，交回空 Vec；全部為負數時，雖然每項都被讀取，也不會 push 任何項。Vec 改變的是輸出表示，沒有替這項工作加入另一種篩選規則。

### 用測試保存整理規則

下面的測試檢查順序、重複值、零、空輸入、全負數，以及原材料沒有被改動。只比較取出的數字與長度，沿用 1.10 的 assertion，沒有要求直接顯示或比較整份 Vec：

```rust
--8<-- "examples/ch03-data/src/list.rs:list-tests"
```

```bash
cargo test -p ch03-data --locked
```

本章 library 共六項測試：四項檢查 3.5 文字區段契約，兩項檢查這裡的清單規則。測試只涵蓋這些材料與契約，不是證明所有未來的清單工作都正確。

## 練習：保留後再套用上限

寫 `capped_nonnegative(values: &[i32], ceiling: i32) -> Vec<i32>`。輸入約定 ceiling 非負：負數仍略過，其餘值若大於 ceiling，就把結果中的那個數字設成 ceiling。仍保留順序、重複次數與原輸入。

用 Vec 保存 120、−5、0、80，借用 as_slice 傳入函式，以 ceiling 100 驗證結果為 100、0、80；原清單的第一項仍應是 120。不要修改借來的原材料，也不要先保留負數後一起截成零，這會改變需求。

完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch03-data/examples/list_solution.rs)，或執行 `cargo run -p ch03-data --example list_solution --locked`。

本章已把 enum 的不同情況、泛型的資料形狀、owned／borrowed 文字與可增長清單連起來。下一章會用 trait 明確表達型別的能力，讓我們能為資料加入適當的比較、複製與顯示契約。
