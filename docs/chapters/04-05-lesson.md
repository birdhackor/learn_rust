# 4.5 pattern 取得的是值，還是借用？

前幾節主要取出 i32。因為 i32 能 Copy，只看數字不容易看見交接後的限制。這次把材料換成第 3.8 節已會建立的 String：QuoteText 可能沒有標籤，也可能以 Label(String) 保存一份文字。觀察同樣的解構外框，如何因為配對的對象不同，讓 binding 取得 String、&String 或 &mut String。

## 配對 owned 資料，String binding 會接手文字

對 owned 資料解構時，一般的名稱 binding 會依它取得的那一部分型別，做 Copy 或 move。`QuoteText::Label(text)` 若取得的是 String，就會把那份文字 move 給 text；它不會因為位在 pattern 裡，就突然產生第二份文字。

下面 consume 接受 owned 的 QuoteText，再把其中的 String 交到 arm 內的 text。main 用 String::new 與 push_str 準備標籤，操作都沿用上一章。

```toml
--8<-- "examples/intro-workspace/patterns/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/patterns/examples/owned_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example owned_patterns --locked
```

輸出為 `接手文字=訂單A`，以及 `沒有文字標籤`。第一筆文字由 Label 移到 text，由這個分支持有到使用完畢；呼叫者已把 QuoteText 交給 consume，後面不再使用原值。

## 取走一個欄位，不代表其餘欄位都取走

再看上一章出現過的文字訂單。這份小例只保留 label 與 count：`let LabelledOrder { label, .. } = order;` 把 String 欄位移給新 label，略過 count。這稱為 **partial move**：只移走一部分。

```rust
--8<-- "examples/intro-workspace/patterns/examples/partial_move.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example partial_move --locked
```

新 label 顯示訂單A，`order.count` 仍能讀到 3，因為這個欄位沒有被移走。不過 order 已缺少被移出的 label，不能再當成完整訂單借給其他函式；以下完整失敗例會得到 **E0382**：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/partial_move.rs"
```

這和把 i32 欄位 Copy 出來不同。要判斷後面能用什麼，須看哪個欄位被 binding、它的型別會 Copy 還是 move，而不能只看是否寫了 match 或大括號。

## match 接受 reference 時，內部 binding 也能借用

如果只是顯示標籤，應像第 3 章一樣接收 `&QuoteText`。當 pattern 寫 Label(text)，而外面的配對對象是 &QuoteText 時，Rust 會自動沿著這層 reference 配對 enum，讓內部 text 成為 **&String**。這種自動安排 binding 的寫法稱為 **match ergonomics**。

對 `&mut QuoteText` 使用相同外框，這裡的 text 則是 **&mut String**，可以呼叫 push_str 修改原文字。`text.as_str()` 仍可借讀內容；不需要為了顯示而把 String move 出來。兩種借用都受第 3.5 節的使用範圍與衝突檢查限制。

```rust
--8<-- "examples/intro-workspace/patterns/examples/borrow_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example borrow_patterns --locked
```

前兩次 show 都顯示訂單A；append 完成後，第三次顯示訂單A：已備妥。同一筆 QuoteText 留在 main，由各次呼叫借讀、借改；前兩次的 text 沒有接手 owned 文字。

以下反例在 shared reference 的 pattern 中取得 &String，卻試圖以 `*text` 交出 owned String，會得到 **E0507**：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/move_from_pattern_borrow.rs"
```

dereference 能找到原值，不會自動給我們移出它的權限。這延續第 3.6 節不能從 &self 交出整筆非 Copy 值的規則。

## ref 是借用 binding，與 & pattern 的角色不同

有時候配對對象是 owned 值，卻只想從其中幾個欄位建立借用。**`ref 名稱`** 讓該 binding 借讀符合的部分；**`ref mut 名稱`** 則讓它借改。這與只寫 `mut 名稱` 不同：單獨 mut 允許重新指派 binding，並不把它變成 reference。

下面 pattern 直接配對 owned 的 order，但以 ref label 得到 &String、ref mut count 得到 &mut i32。它們分別借不同欄位，這個例子可以借讀標籤的同時修改數量，沒有對同一欄位建立衝突存取。

```rust
--8<-- "examples/intro-workspace/patterns/examples/explicit_bindings.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example explicit_bindings --locked
```

數量由 3 改成 4。這次 label 沒有被 move；兩份借用最後使用完後，main 還能直接讀原訂單。

反過來，**reference pattern** 的 `&` 是去配對一個**已經存在的 reference 層**，而不是替 binding 建立借用。例如配對 &QuoteText 時，可以把外層明寫成 `&QuoteText::Label(ref text)`：外面的 & 配對那層 reference，裡面的 ref 再決定 text 要借讀 String。

這個完整形式在下面的 show 中刻意保留，與前面自動借用的版本比較。Clippy 的 match_ref_pats 會建議不必在每個 arm 都寫 &，needless_borrowed_reference 則會建議使用前面較簡短的自動形式。因此只在這個 show 加兩項具名 allow；它們保留當前比較材料，不改變借用規則。

```rust
--8<-- "examples/intro-workspace/patterns/examples/reference_pattern.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example reference_pattern --locked
```

兩次借讀都顯示訂單A。三種位置可以這樣對照：expression 中的 & 建立 reference；pattern 中的 & 配對 reference；pattern 中的 ref 決定新 binding 是 reference。

## 2024 edition 不任意混用自動與明寫的形式

本書使用 edition 2024。當 pattern 已經自動略過 reference 層、轉成借用 binding 時，不能再在那個位置隨意加 ref、ref mut 或 mut；也不能在這種自動配對的內部突然再插入 & pattern。

下面配對對象是 &QuoteText，Label(text) 本來就會讓 text 成為 &String；再加一個 ref 在2024 edition會遭拒絕：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/edition2024_redundant_ref.rs"
```

本書工具鏈的診斷重點是 `cannot explicitly borrow within an implicitly-borrowing pattern`，並標出 ref 不能加在這個自動借用的位置。**binding mode** 指「pattern 預設讓名稱接收值，還是借用」；自動略過 &QuoteText 那層後，預設已經是借用。

這份程式可以改回前面的 Label(text)，使用自動借用；也可以像完整 reference pattern 範例，從外層就明寫 `&QuoteText::Label(ref text)`。後者先明確配對 reference 層，內部才以 ref 選擇借用。舊版範例出現多寫的 ref 時，不要為了消除錯誤而把整份資料改成 owned。

## _ 忽略資料，_name 仍會建立 binding

第 4.1 節已說 _name 仍是 binding。對 String 就能看見這項差異：`Label(_)` 只確認形態，不 Copy、move 或借用那個欄位；之後原 QuoteText 仍能借讀。

```rust
--8<-- "examples/intro-workspace/patterns/examples/ignore_patterns.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example ignore_patterns --locked
```

如果改為 `Label(_text)`，那是名叫 _text 的新 binding，會接手 String；只是名稱開頭的底線避免未使用警告。這份完整失敗例在之後借用原 QuoteText，因 partial move 得到 **E0382**：

```rust
--8<-- "examples/intro-workspace/patterns/fixtures/compile-fail/underscore_binding_move.rs"
```

## 練習與完整答案

建立 Label 形態，文字為訂單B。寫接受 &mut QuoteText 的 append，追加「：完成」，再用接受 &QuoteText 的 show 連續顯示兩次。Missing 的 append 不做修改，show 則顯示沒有標籤。

完整答案沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/patterns/examples/borrow_patterns_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-patterns --example borrow_patterns_answer --locked
```

兩次都顯示 `標籤=訂單B：完成`。append 的 text 是 &mut String，show 的 text 是 &String；兩者都沒有把原資料中的 String 移走。

[下一節：if let 與 while let](04-06-lesson.md)
