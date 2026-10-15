# 9.3 讓函式交回可用的工作：impl Trait 與 lifetime capture

上一節讓函式收到工作並執行它。現在希望函式收到設定值，建立好一份工作，交回去讓呼叫者之後反覆使用。例如保存加數 10，之後把不同輸入各加 10。

這個 closure 需要 capture 設定值，不能只用上一節的 function pointer 表示。它有具體型別，但通常不能在 source 中寫出型別名稱；回傳介面仍必須說清楚呼叫者可以做什麼。

## impl Trait 放在參數與回傳值，誰選型別不同

**`impl Trait`** 在型別位置表示「符合指定 trait 的具體型別」。先看參數位置：`task: impl Fn(i32) -> i32` 接受一份有對應呼叫能力的工作。具體工作由呼叫者提供，函式 body 只能依已宣告的能力使用它。

這裡保留上一節的輸入／輸出契約，但省略另取泛型參數 F 的步驟。它是匿名的型別參數；若要在多個位置表達同一個型別，仍可用原來的具名泛型。這項匿名參數也不能像第 5.1 節的具名參數一樣在呼叫處指定 turbofish。

本節仍用沒有 library 的 `intro-iterators`。每個完整 example 把建立工作的函式與 main 放在同一檔；完整設定如下：

```toml
--8<-- "examples/intro-workspace/iterators/Cargo.toml"
```

先看只執行一次的 evaluate：

這組參數／回傳對照先使用同一種 Fn 契約，讓差別集中在誰提供具體型別。evaluate 本身只呼叫一次，單獨設計這項工作時，FnOnce 就已足夠；此處的 Fn 是本例選定的介面能力。

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_owned.rs:parameter"
```

把 `impl Trait` 放在**回傳位置**，選擇方向反過來：由函式實作選定具體結果型別，呼叫者取得符合這份介面的結果，不能自行改選另一個型別。

這種隱藏具體型別的介面稱為 **opaque type**。這裡隱藏的是原始碼需要寫出的型別細節，不是讓結果沒有型別，也不是把型別改成執行期任選。

`make_adder` 回傳 `impl Fn(i32) -> i32`，承諾結果可接受 i32、交回 i32，且能透過 shared receiver 呼叫。body 選擇一個按值保存 base 的 closure；base 是 Copy 整數，工作不借用函式內會結束的區域變數：

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_owned.rs:owned"
```

| 位置 | 具體型別由誰提供 | 函式與呼叫者知道什麼 |
| --- | --- | --- |
| `task: impl Fn(i32) -> i32` | 呼叫者 | 函式只依宣告的呼叫能力使用輸入 |
| `-> impl Fn(i32) -> i32` | 函式實作 | 呼叫者只依公開的呼叫能力使用結果 |

## 一個回傳介面仍對應一個具體型別

同一函式的所有回傳分支，必須有相同的具體型別。每個不同 closure 表達式都有自己的型別，所以「它們都實作 Fn」不代表能作為同一個回傳結果。

下面故意在兩個分支寫不同的 closure，得到 `E0308`：

```rust
--8<-- "examples/intro-workspace/iterators/fixtures/compile-fail/impl_trait_two_closures.rs"
```

本項工作只需要依設定選擇加或減，不需要兩種資料形狀。因此把分支放進**同一個 closure 的 body**：建立的仍是一種 closure 型別，保存的 subtract 值決定每次執行哪項算術。

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_owned.rs:branches"
```

下面一起觀察參數位置、回傳的加法工作，以及 body 中的加減分支：

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_owned.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example impl_trait_owned --locked
```

```text
參數位置接受工作=6
工廠交回工作=13
保留工作再呼叫=14
同一個 closure body 選分支=20、40
```

evaluate 將 3 交給乘二工作，得到 6；make_adder 保存 10，收到 3、4 後分別得到 13、14。subtract 為 true 時做 30 減 10，否則做 30 加 10，因此是 20、40。不同 base 值可以保存不同設定，但同一個 closure 表達式的型別仍相同；回傳介面保留它能做的工作。

## 回傳工作借用資料，來源仍須有效

現在要交回一份讀文字 byte 長度的工作，讀的仍是既有 String 的內容，不重建另一份文字。輸入使用第 7.1 節的省略形式 `&str`，但來源 lifetime 仍然存在。回傳 closure 用 move 保存這個 shared reference 值；第 9.1 節已說明，按值保存 reference 不會取得被借文字的 ownership。

回傳位置的 `impl Trait` 簡稱 **RPIT**（return-position impl Trait）。本專案採 edition 2024：沒有明確限制時，這個 opaque 型別會自動 capture scope 內的泛型參數，**包含 lifetime 參數，省略名稱的輸入 lifetime 也在其中**。因此下面的隱藏型別可以依賴輸入 lifetime，保存借自輸入的 reference：

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_borrowed.rs:borrowed"
```

這裡的「RPIT capture」是在型別契約上允許隱藏型別依賴哪些泛型參數；第 9.1 節的「closure capture」則是建立值時保存哪些資料或 reference。兩者相關，但不是同一項動作，前者不會配置或複製原文字。

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_borrowed.rs"
```

執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example impl_trait_borrowed --locked`，讀到 `4` bytes，原文字仍是 Rust。來源 String 存在，這份讀取工作才能使用保存的借用。

若把來源移到內層，卻在內層結束後才呼叫回傳工作，便不符合這項依賴，得到 `E0597`。回傳值有 opaque 介面，不會取消原資料的 lifetime 要求：

```rust
--8<-- "examples/intro-workspace/iterators/fixtures/compile-fail/impl_trait_source_too_short.rs"
```

## use 明確列出型別允許依賴的參數

有時隱含 capture 比這次實作真正使用的資料更廣。以下對照刻意保留一個 `&str` 輸入，但工作只交回固定數字 4，不讀輸入；它只為展示回傳契約的差異。

在 edition 2024，省略明確限制仍會 capture 輸入的 lifetime，即使本次 closure 沒有保存該 reference。呼叫端仍依介面允許的依賴檢查，所以這個 fixture 也得到 `E0597`：

```rust
--8<-- "examples/intro-workspace/iterators/fixtures/compile-fail/impl_trait_implicit_capture.rs"
```

要表達較窄的依賴，可用 **precise capture**（精確捕捉）的 `use<...>`。它放在回傳 bound，和第 2.6 節引入名稱的 use statement 是不同用途：

- **`use<'a>`** 明確允許隱藏型別依賴 `'a`。真的保存輸入 reference 的 reader 需要這份來源關係。
- **`use<>`** 不 capture 泛型參數。本例沒有其他型別參數，固定工作也沒有保存輸入 reference，才可以這樣排除來源 lifetime。

`impl Fn() -> usize + use<>` 中的加號，把呼叫能力與 capture 約定放進同一個回傳介面，不是數值相加。下面第一個函式保留來源，第二個明確排除不需要的來源：

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_precise.rs:precise"
```

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_precise.rs"
```

執行 `cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example impl_trait_precise --locked`，兩個結果都是 `4`。第一個工作依賴仍存活的 text；第二個工作在 inside 結束後仍可用，因為其回傳介面明確排除了該依賴。

`use<>` 沒有延長 inside，也不能用來消除一份真實保存的 reference。若實作仍使用那份借用，隱藏型別就需要 capture 相應 lifetime。現行語法若有具名型別參數，也要求 capture 清單列入 scope 中全部型別參數；不能把這個只有 lifetime 的示例推成任意省略 T 的規則。

## 練習：工廠交回一份持有計數的工作

寫 `make_counter`，接收起始 i32，建立可修改的計數，交回每次先加一再回傳計數的 closure。不要借用工廠結束時就消失的區域變數，改讓工作按值保存計數；回傳介面應承諾上一節相應的可修改呼叫能力。

從 1 開始，兩次呼叫得到 2、3；另建一份從 10 開始的工作，第一次得到 11。呼叫者要修改工作保存的狀態，記得讓保存回傳工作的 binding 可修改。數字仍採不溢位的小值。

下面是完整答案；make_counter 與 main 在同檔，設定沿用本頁 manifest：

```rust
--8<-- "examples/intro-workspace/iterators/examples/impl_trait_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-iterators --example impl_trait_answer --locked
```

輸出 `工廠交回的計數工作=2、3`。第一份工作保存自己的 count，兩次呼叫由 1 加到 2、再到 3；第二份工作另從 10 開始，因此第一次是 11。move 讓狀態跟工作一起交回，FnMut 讓之後的呼叫能繼續修改同一份狀態。

你已能在回傳介面保留工作能力，分開 owner、實際借用與 opaque 型別允許的依賴。下一節把工作換成逐筆產生資料的來源，先把取得、推進與結束看清楚。

延伸閱讀：[Rust Book：Returning Closures](https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html#returning-closures)、[繁體 Book：進階函式與 closure](https://rust-lang.tw/book-tw/ch19-04-advanced-functions-and-closures.html)、[Rust Reference：impl Trait](https://doc.rust-lang.org/reference/types/impl-trait.html)、[Edition Guide：RPIT lifetime capture](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html)。

[下一節：Iterator、IntoIterator 與 ownership](09-04-lesson.md)
