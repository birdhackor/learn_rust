# 7.4 method 回傳 reference：分開 receiver 與欄位來源

第 7.3 節的 TextView 保存文字借用，byte_len 則交回獨立數字。現在 method 也要交回文字 reference：外部 String 還留著，模型卻只建在內層。我們能不能在模型結束後，繼續借讀原文字？要先分清楚兩份借用：模型欄位借用原文字，method 的 `&self` 又借用模型。

## 同樣讀欄位，輸出可以連到不同關係

第 3.6 節的 **receiver** 就是 method 的 self 參數；`view.text()` 會為 `&self` 借讀 view。現在有三個對象：

| 對象 | 本例中的角色 |
| --- | --- |
| 外層的 source：String | 持有原文字，內層結束後仍存在。 |
| 內層的 view：`TextView<'a>` | text 欄位借用 source 的文字，模型在內層結束。 |
| 呼叫時的 `&self` | 另一次借讀 view，與欄位借用來源的 `'a` 分開。 |

method 的 lifetime elision 有 receiver 規則：若有 `&self` 或 `&mut self`，省略的輸出 lifetime 就連到這次 receiver 借用。因此 `text(&self) -> &str` 的結果使用期間，view 也必須可以繼續被借用。

若希望輸出直接連到欄位原來源，則明寫 `original(&self) -> &'a str`。這裡的 `'a` 是 TextView 欄位已經宣告的來源參數，不是把 `&self` 固定成 `'a`。

下面兩個 method 都放在 `impl<'a> TextView<'a>` 裡，body 同樣讀 self.text，簽名卻把不同的關係交給呼叫者：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/receiver_only/views/mod.rs:reference-methods"
```

`&self` 的借用不一定只到函式呼叫結束；若輸出連到它，後面的使用也需要維持這份借用。original 的輸出則連到欄位原來源，不需要為了後續結果繼續借用 view。

兩個 body 都能交出 self.text，是因為欄位保存的是可以 Copy 的 shared reference；交出的是借用資訊，沒有 move 外面的 String，也沒有 move 整個 TextView。

## 模型留在內層，來源留在外層

source 在外層保存 `Rust tutorial`。在內層建立 view，先立刻使用 text 的結果，再把 original 的結果交到外層：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/receiver_only/main.rs:receiver-and-source"
```

第一個 println 在 view 還存在時使用 text 的結果，這次 receiver 借用隨該次使用完成。original 交回的 saved 依賴 source 的文字；view 隨內層結束，source 卻仍有效，所以外層可以顯示 saved。這沒有延長 view，也沒有把它的欄位改成 owned 文字。

### receiver 對照的完整程式

這一步仍使用 no-lib package：

```toml
--8<-- "examples/intro-workspace/lifetimes/Cargo.toml"
```

下面是 `examples/receiver_only/views/mod.rs` 的全部內容，沒有後面的雙欄位模型：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/receiver_only/views/mod.rs"
```

這是載入它的完整根檔案，最後用第 6.2 節的文字內容 assertion 檢查 saved：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/receiver_only/main.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example receiver_only --locked
```

```text
借讀模型時的文字=Rust tutorial
模型結束後，原來源仍有效=Rust tutorial
```

### 換成 text，原來源有效仍不夠

若把交到外層的 `view.original()` 換成 `view.text()`，輸出就連到 receiver 借用。完整失敗例如下：

```rust
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/method_receiver_too_short.rs"
```

放進獨立練習專案執行 `cargo check`，得到 **E0597**；這次被指出太早結束的是 **view**。source 的原文字雖然還有效，text 的簽名卻沒有提供一份可以脫離模型借用的輸出。完整 `.expected` 是：

```text
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/method_receiver_too_short.expected"
```

若只需要在借讀模型時使用文字，text 的省略形式已足夠。若模型本來就保存一份可獨立借出的來源 reference，而呼叫者確實要在模型結束後使用，original 的明寫來源契約才能表達這項需要。

## 兩個欄位，可以分別連到兩個來源

再把模型擴成 TextPair，同時保存 first 與 second 的借用。第一份原文字留在外層，第二份只在內層用；最後希望只保留第一份借用。

在 `TextPair<'a, 'b>` 分別宣告兩個 lifetime parameter：first 用 `&'a str`，second 用 `&'b str`。impl 也宣告這兩個參數。byte_len 借讀兩欄、交出長度總和；first_source 與 second_source 則各自交回連到對應來源的 reference：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/src/views.rs:text-pair"
```

使用整份模型的兩個欄位時，兩份來源都要有效。把參數分開命名的用途，是讓 first_source 的**輸出**只依賴第一來源，不必為了保存它而讓第二來源或模型繼續存在。這裡不需要 `'b: 'a`，因為沒有要求第二來源提供第一來源所需的借用。

## 完整 library：此時才載入本章全部功能

到這裡，本章的來源函式與兩種模型都已完成。現在另用 `lifetimes/complete/` 的 `intro-lifetimes-complete` package，整理成可以由呼叫者使用的 library。早期的 no-lib package 保持原來的階段範例。

以下列出這個完整 package 的所有設定與來源。第 2.6 節已教：`src/lib.rs` 是 library 根，`pub mod source;`、`pub mod views;` 分別載入同層 source.rs、views.rs。

完整 manifest：

```toml
--8<-- "examples/intro-workspace/lifetimes/complete/Cargo.toml"
```

完整 `src/lib.rs`：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/src/lib.rs"
```

完整 `src/source.rs` 收入前兩節的函式。測試現在用第 6.2 節的 `&str` 內容比較，包含等長時選第一段，避免只比長度而漏掉來源選擇錯誤：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/src/source.rs"
```

完整 `src/views.rs` 包含本節的模型。第一項測試把 original 的結果帶出模型 scope；第二項則把第一來源的結果帶出第二來源與模型的 scope：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/src/views.rs"
```

完整 `src/main.rs` 先借出較長文字，再由 make_view 建模、顯示內容與長度。`use intro_lifetimes_complete::...` 由 library crate 取得公開項目；package 名稱裡的連字號在 Rust 路徑中改成底線，沿用第 2.6 節的規則：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/src/main.rs"
```

現在執行主要程式與 library 測試，只會載入上面完整提供的程式：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes-complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes-complete --lib --locked
```

run 輸出 `借到較長文字=Python，bytes=6`；六項 library test 應全部通過。測試核對選擇規則與內容，編譯器則檢查這些借用在使用時有效。

### 兩個來源先後結束的完整呼叫者

下面是完整的 `examples/two_models.rs`，沒有其他 helper。它使用上面已提供的 library：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/examples/two_models.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes-complete --example two_models --locked
```

```text
兩段合計 bytes=17
第二來源=note
第二來源與模型結束後，第一來源=Rust tutorial
```

內層先用 pair 讀兩份資料，再只交出 first_source。外層 saved 連到仍有效的 first；不是把已結束的 pair 或 second 保留下來。

## 練習與完整答案

沿用第 7.3 節只有文字借用欄位的 `Label<'a>`。這次提供 original method，讓輸出直接連到欄位來源。在外層建立內容為 Rust 的 String，內層建立 Label 並交回 original，外層確認文字為 Rust。

與第 7.3 節的答案比較：先前把 owned 的數字帶出去，所以來源也能留在內層；這次帶出去的是 reference，所以原 String 必須留在外層。完整答案如下，manifest 及會載入的完整 library 都已在同頁提供：

```rust
--8<-- "examples/intro-workspace/lifetimes/complete/examples/label_answer.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes-complete --example label_answer --locked
```

輸出是 `Label 結束後，仍借讀原來源=Rust`。Label 已結束，來源仍有效；`original` 的簽名讓呼叫者能保留的，正是這份原來源借用。

本章現在能把函式輸出、模型欄位與 receiver 的借用關係分開表達。下一章用這些資料介面，安排失敗資料的表示、格式化與傳遞。

[下一章：失敗是資料，何時才停止流程](08-01-lesson.md)
