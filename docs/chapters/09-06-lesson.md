# 9.6 整理文字與 HashMap：分行、Entry 與三種走訪方式

第 6.6 節已能按名稱計數，第 9.5 節又能把逐筆操作接成流程。現在輸入換成一段多行文字：每行是一個名稱，空行略過，相同名稱合併計數，結果自己擁有名稱。

材料依序是 Rust、Python、Rust、空行、教學；要得到 Rust 兩次、Python 與教學各一次。名稱原樣保留，大小寫與前後空白都參與身分判斷。本頁先完成正確計數；固定報表順序由下一節接續。

本頁仍使用 no-lib 的 intro-iterators，每個 example 都有自己的完整 helper 與 main。完整設定如下：

```toml
--8<-- "examples/intro-workspace/iterators/Cargo.toml"
```

## 先選遍歷的單位：scalar，或整行視圖

**`str::chars()`** 借用文字，回傳 `Item = char` 的 iterator，依序取得 Unicode scalar。它沿用第 5.3 節的 char 意義，不是每次取一個 byte，也不保證每筆就是一個畫面上的完整字形。

下面的文字 `é` 是字母 e 加上結合重音符號，通常顯示成一個字形，但有兩個 scalar、三個 UTF-8 bytes。char 已有標準 Debug，可以檢查它；Debug 把這個結合符號顯示為 **`'\u{301}'`**。這是 Unicode escape，用 scalar 編號表示一個 char，不是一串反斜線、字母與數字；U+0301 是該重音符號的常用編號。

報表需要整行名稱，因此接著選 **`str::lines()`**。它借用文字，回傳 `Item = &str` 的 iterator，每筆是原文字中的一行，不會為每行建立 owned String。

字串 literal 中的 **`\n`** 表示 LF（Line Feed，換行），**`\r`** 表示 CR（Carriage Return，回車）。lines 以 LF 或 CRLF（`\r\n`）分行，結果不含行尾；最後一個行尾不會額外產生一行，空文字產生零行。單獨 CR 沒有跟著 LF 時仍留在內容中，連續行尾之間的空行則是一份空 `&str`。

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_units.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example text_units --locked
```

```text
bytes=3，scalars=2，內容=['e', '\u{301}']
lines=["Rust", "", "Python"]
```

第一份 collect 保存 char 值；第二份保存借自原文字的 reference。這是兩種不同的 Item 與 ownership 關係，不能只看都使用 collect 就認為結果都擁有原文字。

### 用既有 filter 略過空行

**`str::is_empty()`** shared 借用文字、回傳 bool，byte 長度為零就是 true，否則 false；它不修剪空白。lines 的 Item 是 `&str`，filter 參數因此是 `&&str`；`*line` 取得裡面的文字 reference，再以 is_empty 判斷，保留的仍是原 `&str`。

`nonempty_names` 回傳 `impl Iterator<Item = &str>`，使用第 9.3 節的 opaque 介面；輸出 Item 的 reference 依第 7.1 節單一來源規則連到輸入，回傳工作也依 edition 2024 capture 依賴來源。原文字仍必須在走訪時有效。這個 helper 和稍後的 count_names 都保存在本頁 text_counts example 的同一檔：

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_counts.rs:lines"
```

## Entry 把同一個鍵的更新與新增接起來

第 6.6 節用 get_mut／match 區分已有鍵與缺鍵。現在用 **`HashMap::entry(key)`** 取得一份描述這個鍵位置的工作：它 mutable 借用 map、接手候選鍵，回傳 **`Entry`**，代表「已有對應值」或「目前缺鍵」。Entry 不會讓借用規則消失；它保存操作這份 map 的借用關係。

對 `HashMap<String, usize>`，以下 method 接手這份 Entry：

| method | 已有值／缺鍵時的工作 | 回傳 |
| --- | --- | --- |
| `and_modify(callback)` | 有值：執行 callback，交給它 `&mut usize`；缺鍵：不執行 callback | 同一個鍵的 Entry，供下一步使用 |
| `or_insert(value)` | 有值：保留既有值；缺鍵：插入傳入值 | map 中值的 `&mut usize` |
| `or_insert_with(callback)` | 有值：不執行 callback，保留既有值；缺鍵：執行 callback，插入它交回的 usize | map 中值的 `&mut usize` |

**and_modify 的 callback 要求 `FnOnce(&mut usize) -> ()`**：一份 Entry 至多執行它一次，修改參數指向的值後不另交回數字。**or_insert_with 要求 `FnOnce() -> usize`**，只有需要新值時才呼叫。即使提供的 closure 也有重複呼叫能力，這些 method 只需要最低的一次呼叫契約，沿用第 9.2 節的能力包含關係。

先用計數觀察延後建立：新名稱建立值 1，舊名稱加一，另記 callback 真的建立幾個新群組。不是每一筆名稱都執行預設值的 callback：

```rust
--8<-- "examples/intro-workspace/iterators/examples/entry_callbacks.rs"
```

執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example entry_callbacks --locked`，建立新群組的 callback 次數是 2，Rust 次數是 2、Python 是 1。第三筆 Rust 走 and_modify，不再執行 or_insert_with 的建立工作。

候選鍵的 `name.to_owned()` 已在呼叫 entry 前執行，因此重複名稱也先建立候選 String；延後的是值的 callback，不能把這份介面說成會自動省去所有鍵複製。

完整計數函式需要的初值只是 1，不需要額外建立工作，直接用 or_insert(1)。name 是 borrowed `&str`，to_owned 建立獨立鍵，使回傳 map 不借用原文字：

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_counts.rs:counts"
```

### 完整計數程式：來源結束，鍵仍可查詢

下面把原 String 放在內層 block，count_names 交出的 map 留在外層。每個鍵都有自己的 String，因此原文字結束後，仍能用 get 查到結果。全部 helper 與 main 如下：

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_counts.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example text_counts --locked
```

```text
Rust=Some(2)
Python=Some(1)
教學=Some(1)
```

前兩筆輸入 Rust、Python 先各新增1；第三個 Rust 修改已有值成2，空行沒有進入計數，教學再新增1。顯示的是 get 回傳的 Option，所以保留 Some 外框；缺少的 JavaScript 仍是 None。空文字的 lines 沒有下一筆，因此額外 assertion 取得空 map。

如果保留 or_insert 的 mutable reference，再借用 map 更新另一鍵，兩份借用仍會重疊。以下 fixture 得到 `E0499`；要先完成 reference 的最後使用，再操作 map：

```rust
--8<-- "examples/intro-workspace/iterators/fixtures/compile-fail/entry_borrow_overlap.rs"
```

## HashMap 也有三種走訪方式，順序仍沒有保證

對本節的 map，Item 是 tuple，鍵與值各有自己的 ownership：

| 建立方式 | Item | 對原 map 的作用 |
| --- | --- | --- |
| `counts.iter()` | `(&String, &usize)` | shared borrow，保留 map |
| `counts.iter_mut()` | `(&String, &mut usize)` | mutable borrow，可修改值，鍵仍只借來讀 |
| `counts.into_iter()` | `(String, usize)` | 接手 map，逐筆交出 owned 鍵值 |

這些 iterator 都以任意順序走訪。mutable 路徑不交出可修改鍵的 reference，避免鍵改變後破壞第 6.5 節的鍵身分／Hash 一致性。下面先借來累計，再將各值加 10，用名稱查詢後，最後接手 map 取出 owned 名稱與數字：

```rust
--8<-- "examples/intro-workspace/iterators/examples/map_iteration.rs"
```

執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example map_iteration --locked`，各段的名稱輸出順序不保證。原本 Rust 2、Python 1，借改後分別是 12、11；最後 owned 走訪交出這兩份鍵值，合計為 23。前兩段仍由原 map 持有鍵，第三段將鍵 move 給每次迴圈，原 counts 不再可用。順序沒有影響這次逐值加10與加總；固定報表則留到下一節。

## 練習：保留原始名稱的身分

沿用 count_names，把輸入改成 Rust、Rust、rust、尾端有一個空白的 Rust，以及空行。將原文字放在內層，讓外層只接回 map。確認 Rust 為2，rust 為1，`"Rust "` 也為1；空行沒有鍵，map共有三種名稱。

另外確認單獨 CR 仍是名稱內容：`"Rust\r"` 應是一個完整鍵，不能自動當成 Rust。空文字應沒有名稱。這題保留原先的名稱規則，沒有呼叫 trim，也不要求 HashMap 的走訪順序。

完整答案含 nonempty_names、count_names 與 main，沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/iterators/examples/text_counts_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example text_counts_answer --locked
```

```text
Rust=Some(2)
rust=Some(1)
Rust 加尾端空白=Some(1)
```

只有前兩個 Rust 會更新同一個鍵；小寫與空白名稱各建立自己的鍵。答案直接查指定名稱，所以三行的順序由 main 決定，沒有替 map 的任意走訪加上額外保證。原 String 已結束，這些查詢仍成功，因為鍵由 to_owned 建立後交進 map。

現在能選文字走訪單位、分清原樣名稱，並用 Entry 決定已有值或新值的工作。下一節再讓正確計數依完整比較規則排成固定順序的報表。

延伸閱讀：[str::chars](https://doc.rust-lang.org/std/primitive.str.html#method.chars)、[str::lines](https://doc.rust-lang.org/std/primitive.str.html#method.lines)、[Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)、[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)。

[下一節：比較與排序](09-07-lesson.md)
