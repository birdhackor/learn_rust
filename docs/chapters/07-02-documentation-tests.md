# 7.2 從使用者角度測試 library

1.10 的測試寫在 library 裡，能確認數字規則。這一節還要回答另一個問題：**別的 crate 能否照公開的使用方式完成工作，文件中的例子是否仍然正確？**

我們沿用 `ch07-tooling`，把兩筆訂單分別計價後相加。兩筆各買一件商品；每筆商品小計依第一章的規則判運費。單價 80 與 120 時，各筆含運費為 100 與 140，總額應是 240。兩筆單價 150 的商品不能因相加達 300 就免運，因為它們是**兩筆分開的訂單**，總額應為 340。

## 把使用方式寫在函式旁邊

以 `//` 開始的註解到行末為止，供人閱讀，不當作程式執行。**文件註解 `///`** 多一個 `/`，會附在下一個項目上，由 Rust 的文件工具 rustdoc 整理成 API 文件。本例附在公開函式前面。

文件註解內使用 Markdown：文字交代輸入與行為，`# Examples` 是使用例的標題，三個反引號圍住 Rust 程式。這裡的程式是公開函式的呼叫者，使用 `use ch07_tooling::pair_total;` 找到 library，而不是假裝自己位於 library 內。

這樣的可執行 Rust 文件例子叫 **doc test**。`cargo test` 會讓 rustdoc 取出它、編譯並執行；簡單例子沒有自行寫 `main`，工具會提供執行所需的外框。例子裡的 `assert_eq!` 因而也會真的比較結果。

以下 `src/lib.rs` 先用已教的 `use` 引入第一章計價函式，再公開 `pair_total`。文件與實作都保留非負、小數值、結果不能超過 `i32` 範圍的約定：

```rust
--8<-- "examples/ch07-cli/tooling/src/lib.rs:documented-function"
```

`pair_total(80, 120)` 的 body 分別呼叫兩次 `total_with_shipping`，各筆數量都是 1，再相加；沒有把兩筆商品合成一張訂單。文件例子測的是這個公開行為，不是單看編譯是否成功。

### 查看真正生成的文件

`cargo doc` 呼叫 rustdoc 建立文件；`--no-deps` 只替目前選定的 package 建立文件，不連同全部 dependency。`--open` 建置後請求開啟瀏覽器。從 repo 根目錄執行：

```bash
cargo doc -p ch07-tooling --no-deps --open --locked
```

若環境沒有圖形瀏覽器，也可以到 workspace 的 `target/doc/ch07_tooling/index.html` 打開已產生的文件。找到 `pair_total`，應能看到用途、輸入限制與可複製的例子。`cargo doc` 本身沒有替你執行例子；驗證它要使用 `cargo test`。

## integration test 是另一個 crate

**integration test** 是放在 package 的 `tests/` 資料夾中的測試。Cargo 把這一層每份 `.rs` 檔案當成獨立測試 crate；它從外部使用 library 的公開介面，不能直接走進未公開的內部項目。

這與 1.10 的同檔測試不同。library 內用 `crate::...`，其中 `crate` 指向 library 自己；在 `tests/public_orders.rs` 寫 `crate::...` 會指向這份測試 crate。要找受測 library，應使用 `ch07_tooling::...`。

檔案位置如下：

```text
examples/ch07-cli/tooling/
  Cargo.toml
  src/
    lib.rs                  公開 pair_total 與同檔測試
    main.rs                 執行並顯示總額
  tests/
    public_orders.rs        獨立的測試 crate
```

`#[test]` 的契約仍與 1.10 相同；但這裡不再包 `#[cfg(test)] mod tests`，因為 Cargo 已只在測試工作中編譯 `tests/` 的這些目標。`tests/public_orders.rs` 因而可以直接寫：

```rust
--8<-- "examples/ch07-cli/tooling/tests/public_orders.rs"
```

第一項確認兩筆小訂單的結果，第二項確認「每筆判斷運費」的規則。即使有人把 library 改成先加兩筆商品再判門檻，第二項仍能指出公開行為已被改壞。測試的預期答案來自訂單規則，不從待測程式輸出反推。

library 內另保留一項測試，確認一筆免運與一筆未免運的組合。它使用已教的同檔測試形式：

```rust
--8<-- "examples/ch07-cli/tooling/src/lib.rs:unit-test"
```

## 分別執行三種檢查

`cargo test` 在此會包含 library 同檔測試、integration test 與 doc test。這是三種位置與使用角度，不是「有其中一種就足以涵蓋所有規則」。

想單獨查看 integration test 時，`--test public_orders` 選 `tests/public_orders.rs` 這個目標；名稱不帶 `.rs`。`--doc` 則只執行 library 的文件測試。從 repo 根目錄執行：

```bash
cargo test -p ch07-tooling --locked
cargo test -p ch07-tooling --test public_orders --locked
cargo test -p ch07-tooling --doc --locked
```

本例應有一項 library 測試、兩項 integration tests 與一項 doc test 通過。主要 binary 沒有自帶測試時，Cargo 可能另外列出零項測試；零項不會抹除前面的結果。測試名稱輸出順序不是程式行為約定。

如果把文件例子的預期值從 240 改成 200，`cargo doc` 仍能產生頁面，但 doc test 會失敗。反過來，若刪掉例子裡的 assertion，例子仍可能成功執行，卻沒有比較總額。**可執行例子與有意義的驗證需要同時安排。**觀察後把文件改回正確答案。

## 練習：替另一個公開情況留下例子

先自己在文件註解中增加第二個 Rust 例子：一筆單價 300、一筆單價 120，兩筆各買一件。第一筆免運，第二筆加 20，因此總額為 440。使用 `pair_total` 與 `assert_eq!`，再用 `cargo test -p ch07-tooling --doc --locked` 驗證。

需要參考時，這個例子的兩項工作就是呼叫 `pair_total(300, 120)` 保存結果，接著檢查結果等於 440。`use` 與 fence 的放法和第一個例子相同，不需增加新語法。

你現在能從公開使用者的角度確認 library，也能讓文件例子隨程式一起接受檢查。下一節會把「資料已在程式中」改成「資料來自檔案」；讀取的成功與失敗將沿用前面已建立的具體錯誤處理。
