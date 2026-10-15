# 10.2 閱讀 API 文件與測試公開介面

第 2.7 節已把規則寫成測試。現在再問：**另一個 crate 能否依公開文件使用 library，文件中的例子是否真的正確？** 本節用 `intro-cargo-docs` 回答，資料夾是 `examples/intro-workspace/cargo-docs/`；上一節的依賴設定快照仍保留原樣。

例子仍是兩筆訂單、各買一件商品。80 元與 120 元各加 20 元運費，合計 240。兩筆 150 元的訂單也各自判斷運費，總額是 170 + 170 = 340；不能把商品合計 300 當成每一筆都免運。

## 把使用契約寫在公開函式旁

**文件註解 `///`** 將文字附在下一個程式項目上。第 2.7 節的 `//` 是普通註解；多出的第三個 `/` 讓 Rust 的文件工具 **rustdoc** 能將這些文字整理為 API 文件。本例把它放在公開函式 `pair_total` 前面。

文件註解內使用 Markdown。`# Examples` 形成「使用例」標題，三個反引號加 `rust` 圍住 Rust 程式。這份例子從 library 的使用者角度寫 `use intro_cargo_docs::pair_total;`，取得函式後再呼叫。

文件中的可執行 Rust 例子稱為 **doc test**。rustdoc 會取出一般 Rust fence 裡的程式，編譯並執行；這種簡單例子可以省略 `main`，工具會補上執行所需的外框。`assert_eq!` 也會真的執行，並非只供閱讀。

先看這份函式及其文件。它使用上一節已宣告的本機 library；完整檔案稍後也會列出引入 `total_with_shipping` 的 use。

```rust
--8<-- "examples/intro-workspace/cargo-docs/src/lib.rs:documented-function"
```

`pair_total` 分別呼叫兩次 `total_with_shipping`，每次數量都是 1，最後才加總。函式接收兩個 `i32`，回傳 `i32`；輸入採用非負、小整數且計算不溢位的約定。這份 API 沒有替任意負值或極大值另外驗證輸入，因此文件必須把限制寫明。

### 依序讀一份 API 文件

使用這個函式前，先找到公開名稱與簽名，知道要提供兩個單價、會得到一個總額。接著讀行為說明：它計算兩筆分開的訂單，運費也各自判斷。再讀輸入限制，最後以例子核對呼叫形式與答案。只看到回傳型別 `i32`，還不足以知道免運門檻按哪一筆判斷。

之後查 docs.rs 或標準函式庫文件，也用同樣順序：先核對 library 版本，再讀公開項目的簽名、行為、限制與例子。若介面回傳 `Option` 或 `Result`，還要讀哪些情況代表沒有值或失敗；本節函式只有直接回傳數字，不需要自補錯誤處理。

## 產生文件，與執行文件例子是兩件事

`cargo doc` 呼叫 rustdoc 產生文件。`--lib` 選 library，`--no-deps` 不連帶替所有依賴產生文件，`--open` 在產生後請求開啟瀏覽器。在 repo 根目錄執行：

```bash
cargo doc --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-docs --lib --no-deps --open --locked
```

若環境沒有圖形瀏覽器，可以開啟入門 workspace 預設輸出位置的 `examples/intro-workspace/target/doc/intro_cargo_docs/index.html`。若另設 Cargo 輸出位置，則到相應的 `doc/intro_cargo_docs/index.html`。找到 `pair_total`，應看到剛才的行為、限制與使用例。

`cargo doc` 產生頁面，本身不執行例子。要檢查這份文件中的程式，用 `--doc` 選 library 的文件測試：

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-docs --doc --locked
```

應有一項 doc test 通過。它同時確認使用者找得到公開函式、呼叫形式正確，而且這個輸入的總額符合預期。

## integration test 從另一個 crate 使用 library

**integration test** 是放在 package 的 `tests/` 資料夾中的測試。Cargo 將這一層每份 `.rs` 檔當成獨立測試 crate，透過 library 的公開介面完成檢查。它不能直接使用 library 的私有項目。

本節的檔案關係如下：

```text
cargo-docs/
├── Cargo.toml
├── src/
│   └── lib.rs               pair_total、文件例子、同檔測試
└── tests/
    └── public_orders.rs     獨立測試 crate，呼叫公開 pair_total
```

第 2.7 節的 library 同檔測試用 `crate::...` 從 library 自己的根找項目。這裡若在 `public_orders.rs` 寫 `crate::...`，起點則是**這份測試 crate**。因此完整 `tests/public_orders.rs` 使用 library 名稱：

```rust
--8<-- "examples/intro-workspace/cargo-docs/tests/public_orders.rs"
```

兩項測試分別檢查小訂單總額，以及「兩筆分開判斷門檻」。如果有人把實作改成先合併商品、再套一次運費規則，第二項就能抓到答案變成 300 的錯誤。

`#[test]` 的意義仍與第 2.7 節相同。這裡不另包 `#[cfg(test)] mod tests`，因為 Cargo 已只在測試工作中建置這份測試 target。

`--test public_orders` 選這個 integration test target，名稱不帶 `.rs`：

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-docs --test public_orders --locked
```

應有兩項通過。它驗證的是選定的公開行為，並非所有可能的單價。

## 同檔測試仍可留下自己的規則材料

library 內另外保留第 2.7 節已教的同檔測試形式：

```rust
--8<-- "examples/intro-workspace/cargo-docs/src/lib.rs:unit-tests"
```

這裡確認一筆商品小計達 300、另一筆是 120 時，結果為 300 + 140 = 440。`crate::pair_total` 從目前 library 的根取函式。使用已教的 `--lib` 單獨檢查：

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-docs --lib --locked
```

應有一項通過。至此共有一項同檔測試、兩項 integration tests、一項 doc test；三種安排從不同位置使用程式，仍須各自選有意義的材料與預期答案。

如果在自己的練習副本將文件中的 240 暫改為 200，文件仍能產生，但 doc test 會因 assertion 不成立而失敗。若刪掉 assertion，例子又可能成功執行，卻沒有檢查總額。可執行的例子還需要安排明確預期，才能檢查想保留的行為。觀察後將例子還原。

## 完整設定與實際載入的來源

本節 package 只有 library，沒有另一份會先載入命令列功能的 binary。完整 `cargo-docs/Cargo.toml` 與 `src/lib.rs` 如下；`tests/public_orders.rs` 已在上面完整列出：

```toml
--8<-- "examples/intro-workspace/cargo-docs/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/cargo-docs/src/lib.rs"
```

唯一的本機依賴是第 2.6 節的計價 library，其完整設定與載入來源也保存在本頁：

??? example "展開 modules-basic 的完整 manifest 與 library"

    `modules-basic/Cargo.toml`：

    ```toml
    --8<-- "examples/intro-workspace/modules-basic/Cargo.toml"
    ```

    `modules-basic/src/lib.rs`：

    ```rust
    --8<-- "examples/intro-workspace/modules-basic/src/lib.rs"
    ```

    `modules-basic/src/pricing.rs`：

    ```rust
    --8<-- "examples/intro-workspace/modules-basic/src/pricing.rs"
    ```

## 練習與完整答案：增加另一個文件例子

替 `pair_total` 的文件增加第二個 Rust 例子：單價分別為 300 與 120，兩筆各買一件，檢查總額為 440。第一筆免運、第二筆仍加 20 元。先完成自己副本的文件，再用 `--doc` 檢查。

### 可直接驗證的完整答案

答案另存於 `cargo-docs-answer/`，讓本節原始版本保持不變。它使用同樣的計價依賴，只把 library 名稱改成 `intro_cargo_docs_answer`，文件增加第二個完整 Rust fence。

完整 `cargo-docs-answer/Cargo.toml`：

```toml
--8<-- "examples/intro-workspace/cargo-docs-answer/Cargo.toml"
```

完整 `cargo-docs-answer/src/lib.rs`：

```rust
--8<-- "examples/intro-workspace/cargo-docs-answer/src/lib.rs"
```

這份答案的本機依賴仍是本頁已完整列出的 `modules-basic`。執行以下指令，應有兩項 doc tests 通過：

```bash
cargo test --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-docs-answer --doc --locked
```

現在文件不只告訴使用者如何呼叫，也以實際檢查保留選定的公開行為。下一節將把資料來源從程式內的數字改為檔案內容。

[下一節：檔案文字與具體 I/O 錯誤](10-03-lesson.md)
