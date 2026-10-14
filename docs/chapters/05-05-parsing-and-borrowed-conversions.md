# 5.5　解析文字，或借出另一種表示？

第 5.4 節接收已經是 `i32` 的數字，再檢查件數。現在輸入改為文字：`"3"`、`"abc"`、`"101"`。取得 `Count` 必須做兩件事：先把文字解析成整數，再檢查 0 到 100 的件數規則。兩種失敗要留下不同原因。

這一節也會區分另一種「轉換」：有時我們只需要借出現有的文字，或把它保存為 owned 資料。解析內容、取得借用表示與建立 owned 產物，是三種不同的需要。

## FromStr：從 &str 解析一個值

`FromStr` 是標準庫的解析 trait，名字中的 `str` 指文字。它要求 `from_str(text: &str) -> Result<Self, Self::Err>`：借入文字，成功時交出目的型別的值，失敗時交出該實作指定的錯誤。`type Err` 與上一節的 `type Error` 都是 associated type；只是兩個 trait 各自要求的名稱不同。

標準 `i32` 已實作 `FromStr`。這裡使用十進位整數格式，可以在數字前帶 `+` 或 `-`；不能有前後空白、其他字元，數值也必須能由 `i32` 表示。例如 `"3"` 成功，`"abc"`、`"3 "` 或超出上限的 `"2147483648"` 失敗。

它的錯誤型別是標準庫 `std::num::ParseIntError`，保存整數解析失敗的原因，並已提供 `Debug`、`Display`、`PartialEq`。所以我們能用 `match` 決定政策，用 `{error}` 顯示，也能比較成功的 `Result<i32, ParseIntError>`。此刻不需要自行建構這個錯誤或解讀它的內部欄位。

先以 `use std::str::FromStr;` 引入 trait，再呼叫 `i32::from_str("3")`。同一個解析也能由 `str` 的 `parse` 方法呼叫：`"3".parse::<i32>()` 表示目標是 `i32`，回傳同樣的 `Result<i32, ParseIntError>`。`::<i32>` 是第 3.3 節的型別參數指定方式；換成另一個有 `FromStr` 的型別，會使用那個型別的解析規則。

```rust
--8<-- "examples/ch05-errors/examples/integer_parsing.rs"
```

```bash
cargo run -p ch05-errors --locked --example integer_parsing
```

前兩種呼叫都取得 `Ok(3)`。後面三個字串分別失敗；程式照樣繼續處理下一個輸入，沒有把預期的解析錯誤改成 `panic!`。

## 把兩種原因收進一個具體錯誤

`"abc"` 沒有取得整數，屬於 `ParseIntError`；`"101"` 成功取得整數，卻不符合件數規則，屬於 `CountError`。解析 `Count` 的函式需要一個能容納這兩種原因的回傳型別，因此定義 `ParseCountError`：

- `Integer(ParseIntError)`：整數解析失敗。
- `Range(CountError)`：整數已解析，但件數規則不接受它。

兩個 variant 都保存原錯誤，而不是只留一段文字。這使呼叫者之後仍能用 `match` 分類。顯示訊息則沿用第 5.3 節的 `Display`，在原因前補上它發生在哪一個步驟。

```rust
--8<-- "examples/ch05-errors/src/parsing.rs:parse-count-error"
```

第 5.4 節的 `From` 也能用來封裝錯誤：將 `ParseIntError` 或 `CountError` 放進對應 variant，必定成功且保留原原因，因此符合 `From` 的約定。下面兩個實作都以值接收原錯誤，把 ownership 交給 `ParseCountError`。

```rust
--8<-- "examples/ch05-errors/src/parsing.rs:parse-error-from"
```

這裡沒有把所有可能失敗的東西都放進同一種錯誤；只有這項解析工作確實會遇到的兩個來源。

## 為 Count 寫解析規則

現在替 `Count` 實作 `FromStr`，指定 `type Err = ParseCountError`。下面先解析 `i32`；失敗就以 `ParseCountError::from(error)` 封裝並早退，成功再交給 `Count::try_from` 檢查件數。解析方法的回傳型別因此是 `Result<Count, ParseCountError>`。

```rust
--8<-- "examples/ch05-errors/src/parsing.rs:count-from-str"
```

`Count` 裡面只有 owned 的 `i32`，沒有 reference field，成功結果不借用輸入文字。方法簽名也沒有將輸出 reference 的 lifetime 連到 `text`；在這個實作裡，文字只用來取得數字。

下面用 `Count::from_str` 與 `parse::<Count>` 做同一個解析，再處理兩種失敗。輸入若是 `String`，先用已學的 `.as_str()` 取得 `&str` 再呼叫 `.parse()`；整個解析只借用文字，原 `String` 仍可使用。

```rust
--8<-- "examples/ch05-errors/examples/parsing.rs"
```

```bash
cargo run -p ch05-errors --locked --example parsing
```

0 與 100 成功；`abc` 的訊息以「無法解析整數」開頭；-1 與 101 以「件數不符規則」開頭。這個差異是兩個步驟的結果，不是依顯示文字猜測原因。

`Display` 的文字不一定就是解析格式。例如這裡 `CountError` 的「件數 101 超過批次上限100」是給人看的說明，不是能交給 `i32::from_str` 的整數文字。若介面需要「顯示再解析後得到原值」，必須另行約定可接受的格式，不能只因兩個 trait 都處理文字就假定可往返。

## AsRef：不同來源借出同一種資料

有時只想取得文字的 byte 長度，不需要把文字解析成數字。`AsRef<T>` 描述低成本、一定成功的借用式轉換；其 `as_ref(&self)` 借用接收者，回傳 `&T`。`AsRef<str>` 因而提供 `&str`，不建立新的 `String`。

標準 `String` 與 `&str` 都有 `AsRef<str>`。因此可以寫 `fn text_bytes<T: AsRef<str>>(text: &T) -> usize`：依第 4.1 節的 trait bound 要求輸入型別具備這項能力，函式只借入它，再以 `text.as_ref().len()` 取得 byte 長度。呼叫時對 `String` 傳 `&owned`，對已是 `&str` 的變數傳 `&borrowed`；後者的參數是借用那個 reference 值。兩種來源都保留在呼叫者手上。

借出的內容也可能只是某個欄位。下面的 `Label` 用 `id` 與 `text` 一起判斷完整身分，沿用第 4.3 節對全部欄位生成 `PartialEq`、`Eq`、`Hash` 的規則；它的 `AsRef<str>` 只借出 `text`。方法回傳的 reference 借自 `self` 的 field，適用第 4.5 節的 receiver 借用規則。

```rust
--8<-- "examples/ch05-errors/examples/as_ref.rs"
```

```bash
cargo run -p ch05-errors --locked --example as_ref
```

兩個 Label 的 `id` 分別是 1 與 2，所以完整值不相等；借出的 `"Rust"` 文字卻相等。這足以說明為什麼不能把這項欄位投影當成第 4.3 節的 `Borrow<str>`：Borrow 作鍵的借用表示時要求相等與 Hash 保持同一個身分。只看這段文字會把兩個不同鍵當成同一個；AsRef 沒有承諾那種等同關係。

## ToString 與 ToOwned：需要獨立保存時

`ToString` 的 `to_string(&self)` 借用一個值，回傳 owned 的 `String`。標準庫會替具有 `Display` 的型別提供它，所以不用直接實作 `ToString`；實作好 `Display`，就能保存同一套顯示文字。這和 `format!("{error}")` 一樣建立文字結果，只是單一值不必再寫格式字串。

`ToOwned` 則用來從借來的資料建立 owned 產物。它的 associated type `Owned` 指定產物型別：標準 `str` 的產物是 `String`，標準數值 slice `[i32]` 的產物是 `Vec<i32>`。因此 `text: &str` 的 `text.to_owned()` 會複製文字內容到新 `String`；`values: &[i32]` 的 `values.to_owned()` 則複製元素到新 `Vec<i32>`。新產物不依賴原資料的借用存活，原資料也不因此失去 ownership。

與第 4.2 節的 `Clone` 相比，`Clone` 從借用 `T` 得到另一個 `T`；這裡 `ToOwned` 可以把借用的 `str` 內容保存成 `String`、把 slice 內容保存成 `Vec`。只複製一份 `&str` reference 則仍然借用原文字，並沒有取得獨立文字內容。

```rust
--8<-- "examples/ch05-errors/examples/owned_text.rs"
```

```bash
cargo run -p ch05-errors --locked --example owned_text
```

錯誤的顯示文字被保存為 `String`，原錯誤仍可顯示；owned 的文字複本加上「教學」後，原文字仍是 `Rust`；slice 的 owned 產物多放入一個數字，原 array 仍只有兩筆。這些操作把「多一個 reference」與「取得可獨立修改的 owned 資料」分開。

## 檢查解析政策，並串起借用輸入

測試確認三件事：可接受的整數格式與件數界線、兩種原因的分類，以及成功 `Count` 在原文字結束後仍有效。`"2147483648"` 超出 `i32` 的上限，所以在整數步驟失敗；`"101"` 能表示為 `i32`，是在件數步驟失敗。

for 的名稱位置也能放 1.7 的 tuple pattern，例如 `for (text, expected) in ...`：每次把取得的 tuple 分成兩個名稱，在當次 body 使用。match 的 `other => ...` 則用名稱 pattern 接住前面尚未符合的整個值，綁定為 other；這和 `_` 略過而不保存值不同，讓測試能顯示不符預期的完整結果。

```rust
--8<-- "examples/ch05-errors/src/parsing.rs:parsing-tests"
```

```bash
cargo test -p ch05-errors --locked
```

練習：寫 `fn parse_count<T: AsRef<str>>(input: &T) -> Result<Count, ParseCountError>`，只借用不同來源，交回 `Count` 的解析結果。用 `&str` 與 `String` 表示同一個 `"100"`，確認結果相等、原 `String` 仍可使用；再解析 `"101"`，確認它是件數規則失敗。這裡仍使用已學的回傳與 `match`，不需要下一節的 `?`。

??? note "參考答案"

    ```rust
    --8<-- "examples/ch05-errors/examples/parsing_solution.rs"
    ```

    執行：`cargo run -p ch05-errors --locked --example parsing_solution`。
