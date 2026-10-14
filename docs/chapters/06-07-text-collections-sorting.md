# 6.7 整理文字與統計，產生順序明確的報表

4.3 已能按名稱計數，6.6 又能把逐筆操作接成流程。現在把輸入改為一段多行文字：每行是一個名稱，空行略過，相同名稱合併計數；最後按次數由多到少排列，同次數再按名稱由小到大排列。

對 `Rust`、`Python`、`Rust`、空行、`教學`，要得到 `Rust=2`、`Python=1`、`教學=1`。名稱原樣保留，大小寫與前後空白都參與身分判斷；本節沒有新增修剪或名稱標準化政策。

## 先選遍歷的單位：scalar，或整行視圖

**`str::chars()`** 借用文字，回傳 `Item = char` 的 iterator，依序取得 Unicode scalar。它沿用 3.5 的 char 意義，不是每次取一個 byte，也不保證每筆就是一個畫面上的完整字形。

下面的文字 `é` 是字母 e 加上結合重音符號，通常顯示成一個字形，但有兩個 scalar、三個 UTF-8 bytes。char 已有標準 Debug，可以檢查它；Debug 把這個結合符號顯示為 **`'\u{301}'`**。這是 Unicode escape，用 scalar 編號表示一個 char，不是一串反斜線、字母與數字；U+0301 是該重音符號的常用編號。

報表需要整行名稱，因此接著選 **`str::lines()`**。它借用文字，回傳 `Item = &str` 的 iterator，每筆是原文字中的一行，不會為每行建立 owned String。

字串 literal 中的 **`\n`** 表示 LF（Line Feed，換行），**`\r`** 表示 CR（Carriage Return，回車）。lines 以 LF 或 CRLF（`\r\n`）分行，結果不含行尾；最後一個行尾不會額外產生一行，空文字產生零行。單獨 CR 沒有跟著 LF 時仍留在內容中，連續行尾之間的空行則是一份空 `&str`。

```rust
--8<-- "examples/ch06-iterators/examples/text_units.rs"
```

```bash
cargo run -p ch06-iterators --example text_units --locked
```

```text
bytes=3，scalars=2，內容=['e', '\u{301}']
lines=["Rust", "", "Python"]
```

第一份 collect 保存 char 值；第二份保存借自原文字的 reference。這是兩種不同的 Item 與 ownership 關係，不能只看都使用 collect 就認為結果都擁有原文字。

### 用既有 filter 略過空行

**`str::is_empty()`** shared 借用文字、回傳 bool，byte 長度為零就是 true，否則 false；它不修剪空白。lines 的 Item 是 `&str`，filter 參數因此是 `&&str`；`*line` 取得裡面的文字 reference，再以 is_empty 判斷，保留的仍是原 `&str`。

本節在同一個 library 新增 `text_ops` module。`nonempty_names` 回傳 `impl Iterator<Item = &str>`，使用 6.4 的 opaque 介面；輸出 Item 的 reference 依 4.4 單一來源規則連到輸入，回傳工作也依 edition 2024 capture 依賴來源。原文字仍必須在走訪時有效：

```rust
--8<-- "examples/ch06-iterators/src/text_ops.rs:lines"
```

## Entry 把同一個鍵的更新與新增接起來

4.3 用 get_mut／match 區分已有鍵與缺鍵。現在用 **`HashMap::entry(key)`** 取得一份描述這個鍵位置的工作：它 mutable 借用 map、接手候選鍵，回傳 **`Entry`**，代表「已有對應值」或「目前缺鍵」。Entry 不會讓借用規則消失；它保存操作這份 map 的借用關係。

對 `HashMap<String, usize>`，以下 method 接手這份 Entry：

| method | 有值時 | 缺鍵時 | 回傳 |
| --- | --- | --- | --- |
| `and_modify(callback)` | 執行 callback，交給它 `&mut usize` | 不執行 callback | 同一個鍵的 Entry，供下一步使用 |
| `or_insert(value)` | 保留既有值 | 插入傳入值 | map 中值的 `&mut usize` |
| `or_insert_with(callback)` | 不執行 callback，保留既有值 | 執行 callback，插入它交回的 usize | map 中值的 `&mut usize` |

**and_modify 的 callback 要求 `FnOnce(&mut usize) -> ()`**：一份 Entry 至多執行它一次，修改參數指向的值後不另交回數字。**or_insert_with 要求 `FnOnce() -> usize`**，只有需要新值時才呼叫。即使提供的 closure 也有重複呼叫能力，這些 method 只需要最低的一次呼叫契約，沿用 6.3 的能力包含關係。

先用計數觀察延後建立：新名稱建立值 1，舊名稱加一，另記 callback 真的建立幾個新群組。不是每一筆名稱都執行預設值的 callback：

```rust
--8<-- "examples/ch06-iterators/examples/entry_callbacks.rs"
```

執行 `cargo run -p ch06-iterators --example entry_callbacks --locked`，建立新群組的 callback 次數是 2，Rust 次數是 2、Python 是 1。第三筆 Rust 走 and_modify，不再執行 or_insert_with 的建立工作。

候選鍵的 `name.to_owned()` 已在呼叫 entry 前執行，因此重複名稱也先建立候選 String；延後的是值的 callback，不能把這份介面說成會自動省去所有鍵複製。

完整計數函式需要的初值只是 1，不需要額外建立工作，直接用 or_insert(1)。name 是 borrowed `&str`，to_owned 建立獨立鍵，使回傳 map 不借用原文字：

```rust
--8<-- "examples/ch06-iterators/src/text_ops.rs:counts"
```

如果保留 or_insert 的 mutable reference，再借用 map 更新另一鍵，兩份借用仍會重疊。以下 fixture 得到 `E0499`；要先完成 reference 的最後使用，再操作 map：

```rust
--8<-- "examples/ch06-iterators/fixtures/compile-fail/entry_borrow_overlap.rs"
```

## HashMap 也有三種走訪方式，順序仍沒有保證

對本節的 map，Item 是 tuple，鍵與值各有自己的 ownership：

| 建立方式 | Item | 對原 map 的作用 |
| --- | --- | --- |
| `counts.iter()` | `(&String, &usize)` | shared borrow，保留 map |
| `counts.iter_mut()` | `(&String, &mut usize)` | mutable borrow，可修改值，鍵仍只借來讀 |
| `counts.into_iter()` | `(String, usize)` | 接手 map，逐筆交出 owned 鍵值 |

這些 iterator 都以任意順序走訪。mutable 路徑不交出可修改鍵的 reference，避免鍵改變後破壞 4.3 的鍵身分／Hash 一致性。下面先借來累計，再將各值加 10，最後仍可以用名稱查詢：

```rust
--8<-- "examples/ch06-iterators/examples/map_iteration.rs"
```

執行 `cargo run -p ch06-iterators --example map_iteration --locked`，前兩個名稱的輸出順序不保證；修改後 Rust 查到 Some(12)，Python 查到 Some(11)。讀取順序沒有影響這次逐值更新，但對固定報表就需要下一步的明確排序。

## 比較不只回答 true／false：Ordering、PartialOrd 與 Ord

排序需要知道兩項資料的比較結果是較小、相等，還是較大。標準 enum **`std::cmp::Ordering`** 有三個 variant：**Less、Equal、Greater**。它們表示第一個比較對象相對第二個的順序；這個型別有 Debug／PartialEq，可顯示與核對結果。

**`PartialOrd`** 提供部分順序的比較，`partial_cmp` shared 借用兩個值，回傳 `Option<Ordering>`。能比較就是 Some 包住相應順序，不能比較就是 None；None 不等於 Equal。先前的 `<`、`<=` 等操作也由這項能力支持，判斷必須和 PartialEq 的相等規則一致。

**`Ord`** 進一步承諾所有同型別值都能依一致的完整順序比較，也要符合 Eq。它的 `cmp` shared 借用兩個值，直接交回 Ordering。反轉左右，Less 應變 Greater；相等對應 Equal，連續的小於關係也要一致。排序時不能依 callback 第幾次執行任意改變答案。

i32／usize 的 Ord 採數值順序，str／String 採內容 bytes 的字典順序：從第一個不同的位置判斷，前綴相同則短的先。這保留大小寫與內容差異，不是依台灣語言習慣作字典排序。Ord／PartialOrd 已在 prelude，Ordering 則以 `use std::cmp::Ordering;` 引入。

浮點數則有 **NaN（Not a Number，非數值）**，不能和一般值取得這種順序。**`f64::NAN`** 是標準 f64 提供的 associated constant，即型別提供的固定值，直接用 `::` 讀取，不加呼叫括號；本例用它觀察 None。一般 f64 因這種情況沒有 Ord，不能只因能用 `<` 就假定它滿足完整排序契約：

```rust
--8<-- "examples/ch06-iterators/examples/partial_order.rs"
```

執行 `cargo run -p ch06-iterators --example partial_order --locked`，一般浮點比較是 Some(Less)，NaN 比較是 None。若要為浮點資料排序，必須另外決定 NaN 的位置與比較規則；本節報表的名稱與 usize 次數已具備 Ord，沒有這項未定政策。

## sort、sort_by_key 與 sort_by 各交給誰決定順序

slice 的三個排序 method 都需要 mutable borrow，修改原排列，回傳 `()`；空或一項資料也能處理。它們保留比較相等項目的原相對順序，這叫 **stable sort（穩定排序）**。

| method | 如何決定順序 | callback 契約 |
| --- | --- | --- |
| `sort()` | 元素自己的 Ord | 沒有 callback |
| `sort_by_key(callback)` | callback 取得排序鍵，鍵要有 Ord | `FnMut(&T) -> K` |
| `sort_by(callback)` | callback 直接比較兩項 | `FnMut(&T, &T) -> Ordering` |

callback 可能被多次呼叫，不保證每個元素只呼叫一次。本節將 Vec 明確借成 as_mut_slice，再使用這些 slice API。

下面的 tuple 第一欄當排序鍵，第二欄只用來觀察同鍵資料是否維持輸入順序。標準 tuple 各欄位有 Debug／PartialEq 時，也提供相應的整份顯示／逐欄比較能力，因而 Vec 的 Debug 能列出這些 tuple：

```rust
--8<-- "examples/ch06-iterators/examples/sorting_keys.rs"
```

執行 `cargo run -p ch06-iterators --example sorting_keys --locked`，numbers 是 `[1, 1, 2, 3]`；rows 是 `[(1, 30), (1, 10), (2, 20)]`。第一欄相同的兩筆保留 30 在 10 前，沒有偷偷把第二欄也當排序鍵。

sort_by 要回傳 Ordering，不能只交回 bool。這個失敗範例以 `E0308` 指出輸出不符；完整順序還要區分兩項相等：

```rust
--8<-- "examples/ch06-iterators/fixtures/compile-fail/sort_boolean.rs"
```

## 排出完整報表：先次數，再名稱

回到開頭的規則：次數降序，同次數才比名稱升序。先用 owned into_iter，把 map 的 `(String, usize)` 收成 `Vec<(String, usize)>`；排序 callback 借來的是兩筆 tuple，不會接手它們的 String。

tuple 的 `.0` 是名稱、`.1` 是次數。要讓次數大的左資料先出現，先比較 **`right.1.cmp(&left.1)`**。若左是 Rust 的 2、右是 Python 的 1，1 比 2 得到 Less，排序就把**左筆 Rust**放在右筆之前。次數 Equal，才用名稱正常的左右方向比較：

| 左筆／右筆 | 比較結果 | 報表順序 |
| --- | --- | --- |
| Rust 2／Python 1 | 先比右次數 1 對左次數 2，Less | Rust 在 Python 前 |
| 教學 1／Python 1 | 次數 Equal，再比名稱，Greater | Python 在教學前 |

```rust
--8<-- "examples/ch06-iterators/src/text_ops.rs:sort-counts"
```

只按次數排序，Equal 的名稱會保留來源順序；但 HashMap 的來源順序是任意的。因此**stable 不足以保證這份報表每次順序相同**。補上名稱比較後，不同的 map 鍵都有明確位置，這才滿足固定輸出的需要。

把分行／計數／排序接成小函式，回傳 owned 的報表資料。名稱已由 to_owned 保存在 map，現在移入結果 Vec，所以原文字結束後，報表仍然有效：

```rust
--8<-- "examples/ch06-iterators/src/text_ops.rs:report"
```

```rust
--8<-- "examples/ch06-iterators/examples/text_report.rs"
```

```bash
cargo run -p ch06-iterators --example text_report --locked
```

```text
Rust=2
Python=1
教學=1
```

## 練習：較少出現的名稱先顯示

重用 count_names，把報表改為**次數由少到多，同次數仍依名稱升序**。先說清楚哪個比較方向要改，不修改分行、空行或名稱身分規則，也不要為排序 clone 整份名稱。

Rust、Python、Rust、教學 應得到 Python=1、教學=1、Rust=2。用完整 tuple／Vec 比較或逐欄 assertion 核對，並再次確認空文字沒有報表資料。

可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch06-iterators/examples/text_report_solution.rs)，或執行 `cargo run -p ch06-iterators --example text_report_solution --locked`。library 測試另外核對 LF／CRLF／單獨 CR、空行、大小寫與空白名稱，以及不同插入順序都產生相同排序、報表不借用已結束的來源。

本章已把逐筆資料型別、borrowed／owned 選擇、錯誤政策和排序接成可驗證的整理流程。下一章再替這份核心工作接上 Cargo 生態系、檔案與命令列入口。

延伸閱讀：[英文 Rust Book：Updating a HashMap](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-hash-map)、[繁體對應章](https://rust-lang.tw/book-tw/ch08-03-hash-maps.html)、[Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)、[PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)、[Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)、[slice 排序](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by)。
