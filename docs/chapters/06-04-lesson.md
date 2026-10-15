# 6.4 associated type：保留來源與輸出型別的關係

Charge 要求所有實作者都交回 i32，所以共用函式可以直接比較金額。現在換一項需要：接手一份資料來源，取出它保存的內容。數字盒要交回 i32，開關要交回 bool；我們希望共用取出工作，又保留各種結果原來的型別。

## 先保留兩種來源

本節重新定義第 5.1 節形狀相同的泛型 Holder，value 欄位採用 T。SwitchState 則只保存 enabled 的 bool。型別與欄位公開，讓外面的呼叫者能建立它們：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_support/mod.rs:models"
```

`Holder<i32>`、`Holder<String>`、SwitchState 都是具體型別。取出 String 時，還需要把 owned 文字交回呼叫者，不假設未知的 T 都能 Copy。

## 在 trait 裡留下輸出型別的位置

**associated type（關聯型別）** 是 trait 契約中的型別名稱，由各份實作指定對應型別。它不是值欄位，而是讓方法簽名可以表達「依這份實作的選擇交回資料」。

本例定義 Extract，方法叫 extract。**`type Output;`** 宣告名為 Output 的 associated type，現在還沒選具體型別；Output 是我們選的名稱，不是固定關鍵字。

`Self` 代表實作此 trait 的型別；**`Self::Output`** 指這份實作選定的輸出型別。大寫 Self 的路徑找型別，與 `self.value` 存取某個值的欄位不同。方法以 `self` 接手來源，才能取出其中的 owned 內容：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_support/mod.rs:extract-trait"
```

實作者現在要提供兩件事：選定 Output 型別，並寫出符合這個輸出型別的 extract body。

## 在每份 impl 選定結果

SwitchState 的實作寫 `type Output = bool;`，所以 `Self::Output` 就是 bool，body 交回 enabled。

Holder 的實作使用 **`impl<T> Extract for Holder<T>`**。前面的 `<T>` 宣告型別參數，接著用它表示要實作的 `Holder<T>`；這與第 5.1 節泛型 impl 先宣告 T 再使用相同，只是這次實作的是 trait。

`type Output = T;` 讓輸出與 value 欄位的型別相同。T 為 i32 就交回 i32，T 為 String 就交回 String：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_support/mod.rs:extract-impls"
```

對同一份 `Holder<i32>` 的 Extract 實作，Output 已選為 i32。呼叫者不是每次取出時，再把這個輸出改選成 bool。

## 先直接呼叫，看見兩種結果

範例根以 `mod associated_support;` 載入上面模組；方法呼叫也把 Extract 引入 scope，沿用第 6.1 節的規則。下面是 main 的第一段：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated.rs:direct"
```

兩個 extract 分別交回 120 與 true。number、switch 都已交給方法，後面使用的是取回的 amount、enabled；不能因其中欄位是 Copy，就假設整個自訂來源也自動 Copy。

## 泛型函式把輸出關係帶回呼叫者

`S: Extract` 要求來源 S 提供取出能力；**`S::Output`** 表示這個來源的實作選定的輸出型別。共用函式便能保留「輸入哪種來源，就取回它自己的結果」：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_support/mod.rs:extract-generic"
```

它只呼叫契約，不讀 value 或 enabled 欄位。呼叫者仍然能取得不同型別：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated.rs:generic"
```

這份泛型 body 不能任意把結果乘二。它只知道來源可取出內容，還不知道輸出是不是數字。

## equality bound 限制輸出必須是 i32

若某項工作確實只接受數字來源，**`Extract<Output = i32>`** 表示：實作 Extract，且該實作選定的 Output 必須是 i32。這叫 associated type 的 **equality bound**，也就是型別相等要求。

角括號裡的 Output 是 trait 宣告的型別名稱，`= i32` 不是傳入資料值，也不會替既有實作改選結果。這份要求讓 double_number 的 body 能使用 i32 乘法；本節採不會溢位的小整數：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_support/mod.rs:numeric-only"
```

下面完整的預期失敗程式，卻把 Output 為 bool 的 SwitchState 交給它：

```rust
--8<-- "examples/intro-workspace/trait-stages/fixtures/compile-fail/output_bound.rs"
```

編譯診斷 **E0271** 指出預期 i32、實際為 bool。修法依工作需要：通用取出使用 extract_value；真正要乘二就提供數字來源，不能移除 bound 後仍假定輸出能乘二。

## 完整來源與測試

本節的 helper 只有 associated type 相關程式。沒有載入其他章的 Iterator library，也沒有其他隱藏模組。member manifest：

```toml
--8<-- "examples/intro-workspace/trait-stages/Cargo.toml"
```

完整根檔案 `examples/associated.rs`：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated.rs"
```

完整模組 `examples/associated_support/mod.rs`：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_support/mod.rs"
```

其中三項測試核對數字／bool 輸出、乘二規則，與 String 交回後仍能追加使用。最後一項把 `Rust 教學` 的 byte 長度核對為11，沿用第 5.3 節的 UTF-8 長度規則。

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example associated --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example associated --locked
```

```text
直接取出數字=120、布林值=true
共用函式取得數字=120、布林值=true
數字結果乘二=240
```

這裡選 self 接手，是因為工作需要取出 owned 內容；並非所有 trait 方法都應接手資料。

## 練習與完整答案

建立 `FirstPair<T, U>`，保存 first 與 second。實作 Extract，Output 選 T，extract 接手整份來源，只交回 first；second 沒有交回，隨這次來源結束。

先用120、true，取出前顯示第二欄，再用 extract_value 取得第一欄120。另用80、false交給 double_number，得到160。不修改兩個共用函式，也不在泛型實作中假設所有 T 都能相乘。

下面是完整答案，沿用本頁 manifest 與完整 associated_support；開頭的兩項檢查也確認原有來源仍符合契約：

```rust
--8<-- "examples/intro-workspace/trait-stages/examples/associated_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-trait-stages --example associated_answer --locked
```

```text
取出前的第二欄=true
取出第一欄=120
另一份來源的數字乘二=160
```

現在同一項 trait 操作既能保留各來源自己的輸出型別，也能由特定函式進一步要求數字輸出。下一節讓標準 collection 用鍵來查值。

[下一節：HashMap 的建立與查詢](06-05-lesson.md)
