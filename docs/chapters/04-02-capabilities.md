# 4.2 讓資料能檢查、比較與複製

上一節的 `Charge` 回答「這種資料怎麼計算金額」。接著，開發時還會有幾個需要：把整筆資料印出來檢查、判斷兩筆是否相同，以及保留原資料再改另一份。

這些操作也由 trait 描述。標準函式庫已經定義好 `Debug`、`PartialEq`、`Clone` 與 `Copy`；我們要決定資料型別應該提供哪些能力，以及各能力如何處理欄位。

下面分成四個可以獨立執行的 example。各 example 中的 `OrderSummary` 是各自的型別；拆開保存，才能看清楚每一步新增了什麼能力，不會讓後面的設定提前出現在前面的程式。

## 印出整筆資料：`Debug`

之前可以分別印出 `summary.amount`、`summary.count`。若想一次看見型別名稱、欄位名稱與欄位值，可以使用 `Debug`：這個 trait 提供供開發者檢查資料的文字表示。

在 `println!` 的格式中，`{summary:?}` 要求以 `Debug` 印出 `summary`。其中 `:?` 是本節新增的格式設定；原來的 `{}` 和 `{summary}` 並不會自動改用 `Debug`。

對這個有兩個整數欄位的 struct，我們選擇讓編譯器產生實作。`i32` 本來就有 `Debug`，所以編譯器可以逐一印出兩個欄位，再加上 struct 與欄位的名稱。

寫在型別前面的 `#[derive(Debug)]` 就是在要求這項生成工作。和第 1 章的 `#[test]` 一樣，`#[...]` 是附在下一個項目上的 attribute；這次 `derive(...)` 的括號列出要生成的 trait 實作。生成 `Debug` 時，每個欄位型別也必須有 `Debug`，不能跳過不會印出的欄位。

```rust
--8<-- "examples/ch04-traits/examples/debug.rs"
```

從 repo 根目錄執行這個 example：

```bash
cargo run -p ch04-traits --example debug --locked
```

程式輸出：

```text
整筆資料=OrderSummary { amount: 360, count: 3 }
```

這份輸出方便我們把金額與數量一起檢查。`Debug` 的文字格式可以隨程式或版本改變，不適合拿來當需要固定格式的存檔或對外資料。

如果移除 attribute，欄位仍然能個別使用，但整個 `OrderSummary` 就沒有 `Debug`。以下失敗範例只保留這個差別：

```rust
--8<-- "examples/ch04-traits/fixtures/compile-fail/not_debug.rs"
```

編譯器會以 `E0277` 指出 `OrderSummary` 沒有實作 `Debug`。這也區分了「欄位各自會印出」與「整個型別提供印出的契約」。

## 比較整筆資料：`PartialEq`

若兩筆摘要的金額相同，但數量不同，應該算同一筆資料嗎？本例選擇**全部欄位都相等才算相等**，讓比較能檢查完整摘要。

`PartialEq` 提供相等比較能力：`==` 得到是否相等的 `bool`，`!=` 得到是否不相等的 `bool`。前面已用過整數比較，是因為整數本來就有這項能力；自訂 struct 並不自動取得它。

`#[derive(PartialEq)]` 產生逐欄比較的實作，因此每個欄位也必須有 `PartialEq`。若是 enum，兩個值必須先是同一個 variant，該 variant 內的資料也都相等，結果才是 `true`。所以 `Ready { count: 3 }` 與 `Ready { count: 4 }` 仍然不相等。

同一個 attribute 可以列多個 trait，用逗號分隔。下面保留 `Debug`，再加上 `PartialEq`。這也讓 `assert_eq!` 可以比較整筆摘要：它需要 `PartialEq` 來判斷，並需要 `Debug` 在失敗時顯示左右兩個值。

這些相等比較會借用左右兩邊來檢查，不會把兩筆摘要的 ownership 轉移走；比較完仍然可以使用它們。

```rust
--8<-- "examples/ch04-traits/examples/equality.rs"
```

```bash
cargo run -p ch04-traits --example equality --locked
```

程式輸出：

```text
相同欄位=true
同金額、不同數量=false
等待與可出貨相等=false
相同形態、不同數量=false
```

第一組比較固定金額，分別讓數量相同與不同，顯示相同金額下，數量不同仍是不相等的摘要。第二組先換 variant，再只改相同 variant 的數量，顯示 enum 的比較也包含它攜帶的資料。

`derive` 使用型別的欄位結構來生成這種行為。如果應用程式要忽略某個欄位，逐欄比較就不符合需求；不能只加上 attribute，便假設它已經懂得應用程式的「相同」。上一節 `Charge` 的計價方式也需要我們自己寫實作，不能從欄位名稱自動推得。

## 保留原資料，再改另一份：`Clone`

接著給摘要加上文字標籤。`String` 本來就有 `Debug`、`PartialEq` 與 `Clone`：其中相等比較檢查文字內容。借用文字 `&str` 也能以 `Debug` 檢查，並用 `PartialEq` 比較文字內容，所以稍後可以比較 `.as_str()` 得到的借用文字。

依第 2、3 章的 ownership 規則，`String` 沒有 `Copy`，把含有 `String` 的資料指定給另一個變數會轉移 ownership。我們現在要保留原標籤，另外修改一份副本，因此需要一個**明確建立新值**的操作。

`Clone` 提供 `.clone()` 方法。這個方法借用原值，回傳同型別的新值，不消耗原值。對 `String` 而言，它會複製文字內容，讓新 `String` 擁有自己的文字配置；修改新文字就不會修改原文字。

Rust 預先把 `Clone` 這個常用 trait 放進 scope，因此下面呼叫 `.clone()` 不必像上一節的 `Charge` 一樣另寫 `use`。

`#[derive(Clone)]` 對每個欄位呼叫該欄位的 `Clone`，再組成新的 struct，因此每個欄位型別都必須有 `Clone`。下面的 `String` 和 `i32` 都符合：標籤得到獨立的文字副本，金額得到相同的整數值。

不過，**`Clone` 不保證每種型別都複製它所指向的全部資料**。shared reference 的 `Clone` 只複製借用資訊，兩個 reference 仍然借用同一份原資料。`Vec<T>` 的 `Clone` 則建立另一個 `Vec`，對各元素呼叫 `Clone`，所以元素型別 `T` 也必須有 `Clone`。

下面把這兩種情況放在同一段程式：先複製帶有 `String` 的摘要，再複製 `Vec<&str>`。第二個清單的元素是借用文字的 reference；複製清單後，兩份清單的元素仍借用 `original.label` 的文字，沒有因此變成各自擁有文字的 `String`。透過 `.as_slice()[0]` 讀到的 `&str` 可以像第 3 章一樣複製這份借用資訊。

```rust
--8<-- "examples/ch04-traits/examples/cloning.rs"
```

```bash
cargo run -p ch04-traits --example cloning --locked
```

程式輸出：

```text
原標籤=Rust
副本標籤=Rust 副本
兩份清單借到=Rust、Rust
```

修改前的 `assert_eq!` 確認內容相等；只修改副本標籤後，原文字保留 `Rust`，兩筆摘要也因標籤不同而不相等。這正是這次複製 owned 文字的用途。

清單的輸出則對應到另一個契約：它複製了清單與其中的 reference，原標籤仍是兩份清單共同借用的來源。因此判斷 `.clone()` 會得到什麼，要看該型別及其欄位的 `Clone` 行為。

## 讓指定動作也能保留原值：`Copy`

先回到只有兩個 `i32` 欄位的摘要。第 2 章已說明 `i32` 有 `Copy`，指定整數之後，原整數仍然能用；但兩個欄位都是 `i32` 的 struct 不會自動有 `Copy`。

`Copy` 允許 Rust 在本來會轉移值的動作中，隱含複製值本身的資訊。以指定動作為例，有 `Copy` 的摘要在 `let second = first;` 之後，`first` 和 `second` 都是可使用的摘要值；這一步不需要呼叫 `.clone()`。

對這個只包含整數、沒有其他資源需要管理的 struct，可以要求編譯器生成 `Copy`。所有欄位都必須是 `Copy`；此外，具有 `Copy` 的型別也必須具有 `Clone`，所以 attribute 同時列出兩者。反過來，有 `Clone` 並不代表有 `Copy`，剛才的文字摘要就是例子。

```rust
--8<-- "examples/ch04-traits/examples/copying.rs"
```

```bash
cargo run -p ch04-traits --example copying --locked
```

程式輸出：

```text
原數量=3，另一份數量=4
```

指定後只增加 `second.count`，`first.count` 仍是 `3`。這次隱含複製保留了兩份獨立的整數欄位，讓我們可以分別修改。

帶有 `String` 的摘要則不能照做。`String` 持有 heap 文字；若只複製它的持有資訊，兩個 `String` 就會把同一份配置都當作自己擁有。因此 `String` 沒有 `Copy`，需要另一份 owned 文字時使用的是它的 `Clone`。

下面的失敗範例企圖替帶有 `String` 的資料生成 `Copy`：

```rust
--8<-- "examples/ch04-traits/fixtures/compile-fail/string_not_copy.rs"
```

編譯器會以 `E0204` 指出這個型別不能實作 `Copy`，因為 `label` 的型別 `String` 不符合條件。選擇 `Copy` 看的是型別的複製契約，不能只看資料當下放在 stack 還是 heap。

## 練習：修改副本的金額

用剛才的 `LabelledSummary` 完成以下工作：

1. 建立標籤為 `訂單A`、金額為 `280` 的原摘要。
2. 明確複製一份，將副本金額增加 `20`。
3. 檢查原金額仍是 `280`、副本金額是 `300`、兩份標籤文字相同，但整筆摘要不相等。
4. 以 `Debug` 印出兩筆資料，對照檢查結果。

先想清楚原摘要為何不能靠普通指定後繼續使用，以及這次需要哪個 trait 的方法，再開始寫。

可參考[完整答案](https://github.com/birdhackor/learn_rust/blob/main/examples/ch04-traits/examples/capabilities_solution.rs)，並執行：

```bash
cargo run -p ch04-traits --example capabilities_solution --locked
```

這四項能力各自處理不同需要：

| 能力 | 這一節可用的操作 | 生成實作時的欄位行為 |
| --- | --- | --- |
| `Debug` | `{:?}` 印出資料；失敗的 `assert_eq!` 顯示值 | 每個欄位也有 `Debug`，一起形成檢查用文字 |
| `PartialEq` | `==`、`!=`；`assert_eq!` 判斷相等 | 每個欄位也有 `PartialEq`，全部相等才相等 |
| `Clone` | 明確呼叫 `.clone()`，保留原值並建立新值 | 每個欄位也有 `Clone`，逐欄呼叫；是否複製底層資料依型別而定 |
| `Copy` | 指定等動作隱含複製後，原值仍可用 | 本例各欄位為 `Copy`，型別同時提供 `Clone` |

下一節要按文字名稱查詢與累加數量，會再引入查詢所需的鍵契約。

## 延伸閱讀

- [Rust Book：Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [繁體 Rust Book：可衍生的特徵](https://rust-lang.tw/book-tw/appendix-03-derivable-traits.html)
- [std：Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)、[Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
