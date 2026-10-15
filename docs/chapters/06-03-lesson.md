# 6.3 Clone 與 Copy：建立另一份值，保留原資料

上一節已能比較兩份摘要，但兩份文字都是各自建立。現在想保留原摘要，再建立一份可修改的摘要。資料若含有 String，普通指定會 move；這次需要一項明確建立新值的操作。

## Clone 借讀原值，交回一個新值

**`Clone`** 提供 `.clone()` 方法：借讀原值，交回相同型別的新值，原值仍可使用。標準的 String 已有 Clone；它會複製文字內容，讓新的 String 擁有自己的文字空間。

對含 String 與 i32 的摘要，可以用 `#[derive(Clone)]` 產生實作。它逐欄呼叫該欄位的 Clone，再組成新摘要，因此每個欄位都必須提供 Clone。這裡的兩種欄位都符合；上一節的 Debug、PartialEq 也保留，供顯示與比較。

Clone 已在第 5.2 節介紹的 prelude 中，不必像自訂 Charge 額外 use。先看完整例子的前半部做什麼：建立 original、呼叫 `original.clone()` 成為 duplicate，再只向 duplicate 的 label 追加「 副本」。新舊標籤會分別是 Rust、Rust 副本。

## 新容器不代表所有內容都變成 owned 副本

Clone 的行為取決於型別，不能一律理解成「把指向的全部資料都另複製一份」。shared reference 的 Clone 只複製借用資訊，仍借同一份原資料。

`Vec<T>` 的 Clone 建立另一個 Vec，並對每個元素呼叫 Clone，所以 T 也必須有 Clone。若元素是 `&str`，新清單的元素仍是借用文字的 reference，並未變成擁有文字的 String。

下面的完整程式先觀察 String 摘要的獨立修改，再建立 `Vec<&str>` 借讀 original 的標籤，複製這份清單。兩份清單都借到 original 的 Rust；原標籤必須在這些借用使用時保持有效，仍受第 3.5 節的限制：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/cloning.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example cloning --locked
```

```text
原標籤=Rust
副本標籤=Rust 副本
兩份清單借到=Rust、Rust
```

修改 duplicate 前整筆相等，修改後整筆不相等；只有金額仍相等。清單則多了一份容器與 reference 元素，兩份元素仍借同一份文字。判斷 clone 的結果，要看這個型別及其欄位的契約。

## Copy 讓指定也保留原值

再回到只有兩個 i32 欄位的摘要。第 3.2 節說過：整數已有 Copy，自訂 struct 不會因欄位都是整數就自動 Copy。

**`Copy`** 允許在原本會 move 的動作中，隱含複製值本身的資訊。對這個數值摘要，`let second = first;` 之後兩份摘要都能使用，不必另外呼叫 clone。這項複製不能由實作者插入另一段任意工作。

產生 Copy 時，所有欄位都必須是 Copy；型別也必須提供 Clone。因此下面同時列出 `Clone, Copy`。反過來，有 Clone 不代表有 Copy，剛才帶 String 的摘要就是例子。

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/copying.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example copying --locked
```

```text
原數量=3，另一份數量=4
```

second 的數量加一，不會修改 first 的數量。這次保留的是兩份可獨立修改的數字欄位；Copy 的判準是型別的複製契約，不能從實際資料放在 stack 還是 heap 推出。

## String 欄位不能產生 Copy

String 管理自己的文字配置；若只把這份管理資訊當作另一個獨立 String 複製，兩份值便會重複負責同一份資源。因此 String 提供能建立獨立文字的 Clone，沒有 Copy。

下面是完整的預期編譯失敗程式，企圖為帶 String 的摘要產生 Copy：

```rust
--8<-- "examples/intro-workspace/trait-stages/fixtures/compile-fail/string_not_copy.rs"
```

放入獨立練習專案執行 `cargo check`，得到 **E0204**，指出欄位 String 不符合 Copy。將資料的放置位置換掉，不會替它取得這項能力。

本頁各成功例與失敗例都完整自足，成功例使用同一份 member 設定：

```toml
--8<-- "examples/intro-workspace/trait-stages/Cargo.toml"
```

## 練習與完整答案

建立標籤為「訂單A」、金額280的 LabelledSummary。明確複製一份，將副本金額增加20；確認原金額280、副本金額300、標籤文字相同，但整筆摘要不相等，再用 Debug 顯示兩份。

先從本次需要判斷：要保留原有 owned 文字摘要，必須使用它的 Clone，不能普通指定後繼續用原值。

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/cloning_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example cloning_answer --locked
```

```text
原資料=LabelledSummary { label: "訂單A", amount: 280 }
更新資料=LabelledSummary { label: "訂單A", amount: 300 }
```

這次只改副本金額，文字仍相等；原資料與新資料各自保留。下一節回到 trait 的定義，讓不同實作者可以交回不同型別的內容。

[下一節：associated type](06-04-lesson.md)
