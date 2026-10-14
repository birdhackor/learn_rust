# 3.3 泛型：保留資料形狀，替換其中型別

我們已經用 Some 保存數字，也用 Ok／Err 保存數字或自訂錯誤。這些外框不必只服務數字；例如「有一個 bool 設定，或沒有設定」同樣適合 Some／None。需要保留的是結果形狀，改變的是它保存哪種型別。

**泛型（generics）** 讓型別或函式定義使用**型別參數**：先替要填入的型別取名稱，在使用時決定具體型別。值參數收到 120、80 這種資料，型別參數則代表 i32、bool 這種型別，兩者扮演不同角色。

## 型別參數不是執行時的任意資料

先用一個小資料盒建立語法。`struct Holder<T> { value: T }` 在型別名稱後宣告 T，讓 value 欄位採用這個型別參數。T 來自 type，這裡只是常用名稱，不是 Rust 特殊型別。

使用時，`Holder<i32>` 表示 value 是 i32 的盒子，`Holder<bool>` 表示 value 是 bool 的盒子。角括號裡放的是型別。這是兩個不同的具體型別，不是同一個盒子能在執行時任意把數字換成布林值。

泛型函式把宣告放在函式名稱後：`hold<T>`。參數 `value: T` 與回傳 `Holder<T>` 使用同一個 T，表示輸入與盒中資料的型別一致。函式接收這個值，再把它交進盒子；是否 Copy 或 move 仍看該具體型別，泛型沒有取消 ownership。

對尚未確定的 T，這個函式只做「保存原值」。不能假定所有 T 都能相乘或顯示，因此沒有對 T 算術或在 body 裡列印它。等到使用位置確定是 i32 或 bool，才沿用前文已知的數字／布林操作。

## 型別推導與明確指定

`hold(120)` 能從輸入及其他型別要求推導 T。也能在接收變數寫 `: Holder<i32>`，表示所期待的具體型別；省略角括號不代表它沒有型別。

要直接在呼叫指定型別參數，寫 `hold::<i32>(80)`。`::<...>` 俗稱 **turbofish**：函式名稱後的這一段放型別參數，後面的普通小括號仍放值參數。它在推導線索不足，或需要把選擇說得明確時有用；已經能清楚推導時，不必每次都寫。

```rust
--8<-- "examples/ch03-data/examples/generic_holder.rs"
```

```bash
cargo run -p ch03-data --example generic_holder --locked
```

```text
數字盒=120，布林盒=true
指定型別的數字盒=80
```

這裡只定義一次 Holder 的欄位形狀與 hold 的保存規則。兩個使用位置選不同型別，但每個盒子的欄位都維持自己已決定的型別。泛型減少的是「只有型別不同的重複定義」，不是讓型別檢查變成可有可無。

## 同一個參數，同一個型別

若資料模型有兩個欄位都寫 T，就表示同一筆資料的兩個欄位必須採相同型別。下面的 Pair 定義兩個 T，建立時卻一邊放數字、一邊放 bool，因此不能編譯：

```rust
--8<-- "examples/ch03-data/fixtures/compile-fail/generic_same_type.rs"
```

錯誤碼 `E0308` 表示型別不符。修正方式依原需求而定：若兩欄本來同型別，就修正輸入；若它們可以不同，就宣告兩個型別參數，例如 `Pair<T, U>`，讓 first 採 T、second 採 U。逗號分隔型別參數，與函式的多個值參數相似。

## 現在寫出 Option 與 Result 的完整型別

Option 使用一個型別參數：**`Option<T>`**。T 決定 Some 中的型別，None 則沒有附帶值。因此 `Option<i32>` 與 `Option<bool>` 的外框同樣是 Some／None，但它們是不同具體型別。

Result 使用兩個型別參數：**`Result<T, E>`**。T 是 Ok 中的成功型別，E 是 Err 中的錯誤型別。`Result<i32, DivideError>` 表示成功時拿到 i32，失敗時拿到我們定義的 DivideError；E 只是 error 的常用參數名。

這讓函式能在簽名直接說明完整結果：接收兩個 i32，回傳 `Result<i32, DivideError>`。參數數字與成功／失敗的型別，都在呼叫前已經明確。

None 沒有內容可供推導，因此若直接建立缺值，寫 `let missing: Option<i32> = None;`，告訴編譯器這是哪種 Option。前一節透過 if 中的 Some 數字提供線索，現在則在缺值本身加上具體型別。

```rust
--8<-- "examples/ch03-data/examples/generic_results.rs"
```

```bash
cargo run -p ch03-data --example generic_results --locked
```

你會依序看到缺少數字報價、取得報價 120、設定值 true，以及整數商 6、零除數失敗。divide 的 Result 不需由呼叫者猜測是數字、布林值還是錯誤訊息；成功與失敗各自的型別已在函式簽名說明。

本例同樣只驗證小整數與零除數，沒有增加邊界運算政策。現在也不直接比較整個 Option 或 Result；pattern 取出已教的數字後，才能沿用目前已教的數值檢查。

## 練習：兩種角色不必相同型別

定義 `Pair<T, U>`，欄位為 first、second。再寫 `pair<T, U>`，接收這兩種值並回傳同樣型別的 Pair。建立 `Pair<i32, bool>` 保存金額 120 與是否可用 true，對各欄位使用已教的 assertion，最後各別顯示。

先說清楚輸入的 T、U 分別對應哪個欄位，再寫回傳型別；不要把兩個型別參數當成兩筆執行期數字。完成後可對照 [參考程式](https://github.com/birdhackor/learn_rust/blob/main/examples/ch03-data/examples/generic_solution.rs)，或執行 `cargo run -p ch03-data --example generic_solution --locked`。

你已能閱讀帶型別參數的標準資料模型。下一節開始處理文字，會分開「擁有整份文字」與「只借用其中的內容」，再把 Option 用在安全取得文字的結果上。
