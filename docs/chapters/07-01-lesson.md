# 7.1 回傳 reference：用 lifetime parameter 說明來源

第 3.5 節已知道：使用 reference 時，原資料必須仍有效。第 5.3 節的 `text.get(start..end)` 還讓我們取得借來的文字片段。現在要自己寫一個函式，從兩段文字中選一段借給呼叫者，不另建 String。呼叫者需要知道：保存這個結果時，哪份原文字必須繼續有效？

## 一個輸入來源時，可以省略標註

先只接收一段文字，原樣交回借用。`same` 沒有建立新文字，也沒有接手 String：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/single_source.rs:same"
```

這裡輸入、輸出都是 `&str`。回傳值必須連到輸入的借用來源，呼叫者不能在來源結束後繼續使用它。

這種簽名可以不寫 lifetime annotation（標註）。Rust 的 **lifetime elision** 是一組省略標註時的推導規則：當函式只有一個輸入 lifetime，省略的輸出 lifetime 就連到它。這不是 reference 沒有 lifetime，而是這個來源關係已足夠明確。

本章先使用 `lifetimes/` 的 `intro-lifetimes` package。它沒有 library；每個 example 只載入自己明寫的 module。完整 manifest 如下：

```toml
--8<-- "examples/intro-workspace/lifetimes/Cargo.toml"
```

下面是這個單一來源例子的全部程式：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/single_source.rs"
```

從教材 repo 根目錄執行：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example single_source --locked
```

輸出是 `借回原文字=Rust`。source 留在 main，saved 的最後一次使用也在 main 內，因此借用仍有效。

## 兩個可能來源，不能只靠省略規則

接下來要選 **byte 長度較長**的一段；等長時選第一段。沿用第 5.3 節的 `str.len()`，這不是比較中文字數，也不是比較 lifetime。

若直接照抄兩個 `&str` 參數，再回傳 `&str`，會得到這份完整失敗例：

```rust
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/ambiguous_output.rs"
```

放進第 1.2 節的獨立練習專案 `src/main.rs`，執行 `cargo check`，會得到 **E0106**：輸出的借用要連到哪個輸入，簽名沒有說明。對一般函式，省略的各個 reference 輸入先分別具有自己的 lifetime；這裡有兩個可能來源，不能套用前面的唯一輸入規則。

函式 body 雖然有選擇分支，仍需要用簽名把對呼叫者的來源要求寫出來。這份 fixture 保存的完整 `.expected` 是：

```text
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/ambiguous_output.expected"
```

## 用同一個參數連起兩個候選來源與輸出

**lifetime parameter** 是用來描述借用有效範圍之間關係的泛型參數。它以單引號開頭，例如 `'a`；與第 5.1 節的型別參數一樣，先在函式名稱後的尖括號宣告，再用於簽名。不過它描述的是有效範圍關係，不是資料型別。

`&'a str` 的 `'a` 放在 `&` 與 str 之間。以下簽名把兩個候選輸入與輸出都標成 `'a`：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_basic/source/mod.rs:longer"
```

對這個函式，呼叫者使用輸出所需的區間，必須能同時由兩個候選來源支持。因為簽名允許交回 first，也允許交回 second，所以不能只保證其中一份原資料有效。

`'a` 不會要求兩份 String 同時建立、同時結束；也不會把哪個值延長到另一個值的 scope。編譯器依每次呼叫及後續使用檢查這個共同要求。函式 body 則必須遵守簽名，不能突然交回指向已結束區域資料的 reference。

## 把結果用在兩份來源都有效的位置

outside 先保存 `Rust tutorial`，內層 inside 保存 `hi`。在內層選擇並顯示結果，兩份來源都還有效：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_basic/main.rs:inside"
```

| 呼叫的 first、second | byte 長度 | 選到的來源 |
| --- | --- | --- |
| outside、inside | 13、2 | first，也就是 outside |
| inside、outside | 2、13 | second，也就是 outside |

交換參數讓兩個分支都實際運作。無論本次選哪一個，selected、reversed 都在 inside 結束前用完，符合相同的兩來源契約。

### 這個階段的全部檔案

這個 folder example 的根是 `examples/source_basic/main.rs`。其中的 `mod source;` 載入旁邊的 `source/mod.rs`，`use crate::source::...` 仍依第 2.6 節從**這個 example 的根**找到函式。完整 helper 只有 same 與 longer：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_basic/source/mod.rs"
```

完整呼叫者如下，設定沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_basic/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example source_basic --locked
```

```text
單一來源=Rust tutorial
第一段較長=Rust tutorial
第二段較長=Rust tutorial
等長選第一段=ab
```

最後使用不同內容、同為 2 bytes 的 `ab` 與 `CD`。印出 ab 才能觀察到「等長選第一段」；只檢查長度 2 無法分辨兩個來源。

## 標註不能讓過短的來源繼續存在

把結果帶到內層外面，情況就不同。下面是完整失敗例，函式已經有 `'a`，不是忘了補標註：

```rust
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/source_too_short.rs"
```

執行 `cargo check` 會得到 **E0597**：inside 在 block 結束時結束，selected 卻在外面還要使用。即使本次 outside 比較長，呼叫者仍依 longer 的簽名檢查兩個可能來源，不能把本次內容的比較結果當成較窄的來源契約。

完整 `.expected` 如下：

```text
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/source_too_short.expected"
```

若需求仍是兩段任選一段，就讓結果在兩份來源都有效時用完，如前面的成功例。下一節再改變需求：只借第一來源，或交出獨立的 owned 文字。

## 練習與完整答案

寫 `shorter`，借回 byte 長度較短的一段，等長時選第一段。用 outside 的 `Rust tutorial` 與內層 inside 的 `hi`，交換參數測兩次；都應印出 hi。再用 ab、CD 檢查等長規則。結果仍可能來自任一參數，因此保留兩來源的 lifetime 關係。

完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/shorter_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example shorter_answer --locked
```

兩次長度檢查成功後，輸出依序是 `第一段較長時選到=hi`、`第二段較長時選到=hi`、`等長選第一段=ab`。選長或選短只改變 body 的選擇規則；回傳借用時需要哪份來源繼續有效，則由簽名表達。

[下一節：單一來源、owned 結果與 outlives](07-02-lesson.md)
