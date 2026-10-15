# 7.2 調整借用來源：單一來源、owned 結果與 outlives

第 7.1 節的 longer 可以交回任一輸入，所以輸出使用期間，兩個候選來源都必須有效。現在沿用外層 outside、內層 inside，分清兩種不同需要：有時只需要借第一段；有時來源即將結束，卻要留下可獨立使用的文字。改變需要後，函式的簽名與工作也跟著改變。

## 只回傳第一段，就只連到第一來源

先把規格改成「一定回傳 first」。第二參數仍可以傳入，但這個小例刻意不使用它；參數位置的 `_` 沿用第 4 章的忽略 pattern，不建立名稱。

用 `'a` 只標 first 與輸出，第二參數的 `&str` 不與 `'a` 連在一起：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_scope.rs:first-text"
```

這份契約讓呼叫者知道：回傳借用依賴 first 的來源；第二段不必為了**回傳結果的後續使用**繼續有效。兩個參數在呼叫當下仍然都必須是有效的 reference。

因此可以讓 inside 在內層結束，只把借自 outside 的結果帶出去：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_scope.rs:save-first"
```

outside 還留在外層，因此 saved 可以在 block 外顯示。這沒有讓 inside 延長存在，也沒有替 longer 放寬要求：first_text 改成只回傳第一段，已經是另一個規格。

### 完整對照程式

本節各 example 沿用這份完整 manifest，沒有 library 或額外依賴：

```toml
--8<-- "examples/intro-workspace/lifetimes/Cargo.toml"
```

下面把兩來源的 longer 與只接第一來源的 first_text 放在同一個完整程式，沒有其他 helper：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_scope.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example source_scope --locked
```

```text
兩來源契約，內層使用=Rust tutorial
只接第一來源，外層使用=Rust tutorial
```

兩行文字相同，能使用的位置卻不同；這來自簽名所說的來源關係，不是本次印出的內容。

## 區域文字即將結束，就交出 owned 結果

若函式自己建立一份 String，它在函式結束時就要交接或清理。下面試圖交回借自這份區域文字的 `&str`；即使宣告 `'a`，仍不能讓 inside 留下：

```rust
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/local_output.rs"
```

放進獨立練習專案執行 `cargo check`，會得到 **E0515**：不能交回借自區域變數的值。`'a` 只描述關係，不會建立可以繼續借用的文字空間。這份完整 `.expected` 是：

```text
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/local_output.expected"
```

如果需要留下內容，可以改交出 String。以下 copy_text 借讀來源，用第 3.8 節的 `String::new()` 與 `push_str` 建立另一份 owned 文字，再把它 move 給呼叫者：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/owned_output.rs:copy-text"
```

copy_text 是自訂函式名；它的 body 明確複製文字內容，回傳型別是 String。這與只複製一份 `&str` 視圖資訊不同：結果持有自己的文字資源，需要儲存另一份內容。

呼叫它後，inside 可以結束；外層保存的是新 String，不是指向 inside 的借用：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/owned_output.rs:source-ends"
```

完整程式還做第二個觀察：從 original 複製後，對 original 追加文字，copied 的內容仍保持 Rust。copy_text 的輸出不再借用 original，所以原資料的修改不必等待 copied 用完。

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/owned_output.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example owned_output --locked
```

```text
來源結束後，owned 結果=Rust
原資料追加後=Rust tutorial
獨立結果仍是=Rust
```

借回來源與建立 owned 結果都能提供文字，責任不同。來源能配合使用區間時，可以直接借讀；需要與來源分開保存、修改時，獨立文字讓兩者各自管理內容。

## `'_`：明寫這裡有 lifetime，仍交給省略規則推導

有時希望在型別位置明確標出 lifetime，但不需要替它命名。**placeholder lifetime** 寫成 `'_`，意思是讓 Rust 依省略規則推導這裡的 lifetime；它不是新的具名參數，也不會延長資料。

把第 7.1 節的 same 改寫如下：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/placeholder.rs:placeholder"
```

這裡仍只有一個輸入 lifetime，所以輸出連到它。兩個 `'_` 並不是靠「字面相同」指定共同來源，而是由唯一輸入規則得到這個關係。若改成兩個各自省略 lifetime 的輸入，輸出仍不能只寫 `'_` 就自動選出來源。

這個階段的完整程式如下；後面 struct 的型別位置也會用到 `'_`：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/placeholder.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example placeholder --locked
```

輸出是 `推導後仍借原文字=Rust`。原 String 仍在 main；把省略的位置寫得更明顯，沒有改變 ownership 或借用來源。

## outlives：第二來源至少能支持輸出需要的區間

現在加入第 5.2 節的 Option：優先文字可能缺少，缺少時才借用 backup。`Option<&str>` 是「可能有一份文字 reference」；`Some(text)` 有借用，None 沒有。`Some("")` 仍是有值，只是內容為空，不能當成缺少。

我們把優先來源標成 `'a`，backup 標成 `'b`，回傳 `&'a str`。要讓 None 分支也能交出 backup，必須保證 `'b` **至少涵蓋** `'a` 所需的區間。

這種關係稱為 **outlives bound**，寫成 `'b: 'a`。冒號左邊的 `'b` 至少與右邊 `'a` 一樣長，也可以更長；不要求嚴格更長。

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/outlives.rs:fallback"
```

Some 分支直接交回 `'a` 的借用；None 分支原本有 `'b` 的借用，因為 `'b: 'a`，也足以支持輸出要求。這裡仍可用一個共同參數寫出類似第 7.1 節的契約；分成兩個名稱，是把「第二來源至少支持第一段所需區間」這項關係明寫出來，不是讓資料活得更久。

完整呼叫者先在外層建立 backup，再在內層建立 preferred。結果都在內層用完：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/outlives.rs"
```

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example outlives --locked
```

```text
有值選到=Rust
缺值選到=backup
有值但為空，長度=0
```

前兩行確認兩個分支的來源，第三行確認 Some 空文字不會改選 backup。backup 可以提供內層所需的借用；這不是要求它的整個 scope 必須恰好等於 `'b`。

如果拿掉 bound，簽名只剩兩個沒有關係的參數，函式就不能保證 None 分支的 backup 能支持 `'a`。以下是完整失敗例：

```rust
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/missing_outlives.rs"
```

執行 `cargo check`，診斷會指出 `lifetime may not live long enough`，並建議補上 `'b: 'a`。完整 `.expected` 保存這個穩定字串：

```text
--8<-- "examples/intro-workspace/lifetimes/fixtures/compile-fail/missing_outlives.expected"
```

## 練習：第一段空時，才選第二段

寫 `first_nonempty`：第一段非空就回傳第一段；第一段為空才回傳第二段；兩段皆空時仍回傳第二段的空片段。這次沒有 Option，判斷的是現有文字的長度，不是資料是否缺少。

它仍可能交回任一參數，因此兩輸入與輸出使用共同的 lifetime parameter。完整答案中的核心如下：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_complete/source/mod.rs:first-nonempty"
```

### 本節完成後的完整來源與答案

下面把已教完的來源函式收在 `examples/source_complete/source/mod.rs`。同檔案的四個 test 檢查兩側輸入、owned 內容獨立、Some 空文字與 None，以及這題的空文字規則；只檢查已學的數字與 bool。

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_complete/source/mod.rs"
```

這是完整根檔案。`mod source;` 只載入上面的 helper；不會載入下一節的 struct：

```rust
--8<-- "examples/intro-workspace/lifetimes/examples/source_complete/main.rs"
```

設定沿用本頁 manifest，執行及測試這個 example：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example source_complete --locked
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-lifetimes --example source_complete --locked
```

run 的最後三行是 `第一段空，選到=Rust`、`第一段非空，選到=Rust`、`兩段皆空，長度=0`；test 的四項檢查應全部通過。第一段非空時，即使第二段 Python 更長，也保留 Rust，因為這題的需要是找第一段非空文字，而不是比較長短。

現在能依需要分辨：簽名連到哪些來源、能否借到外層，以及是否應交出 owned 資料。下一節把 reference 存進 struct，讓資料模型也能表明自己依賴的來源。

[下一節：struct 保存 reference](07-03-lesson.md)
