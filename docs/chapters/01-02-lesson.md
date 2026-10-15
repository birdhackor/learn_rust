# 1.2 第一個可執行程式

這一節要讓終端機顯示 `Hello, Rust!`，並分清「檢查成功」和「程式已執行」。請先在教材 repo **外面**選一個練習資料夾，打開終端機。以下指令都在終端機輸入，不是 Rust 程式碼。

Cargo 管理的一份專案叫 **package**，它的設定檔是 `Cargo.toml`。`cargo new` 會建立初始設定與原始碼；`cd` 切換目前工作的資料夾：

```bash
cargo new intro-hello
cd intro-hello
```

`intro-hello` 是這次選的專案名稱。如果已有同名資料夾，換一個新名字。打開 `src/main.rs`；`src` 是 source 的縮寫，`.rs` 表示 Rust 原始碼。

## 先讀懂三行程式

我們把一段有名稱的工作叫作**函式**。可執行程式的入口是 `main` 函式；`fn main()` 中，`fn` 定義函式，`main` 是名稱，空的 `()` 表示這次沒有輸入參數。`{`、`}` 圈起函式的內容，裡面的工作按順序執行。Rust 用括號界定範圍，縮排方便人閱讀。

要顯示一行固定文字，寫 `println!("文字");`。雙引號中直接寫出的文字叫**字串 literal**；literal 就是直接寫在原始碼裡的值。`println!` 將文字顯示在終端機並換行；`!` 表示這是 **macro**，一種在編譯時處理的特殊語法呼叫。現在只需依這個形式使用它，macro 的內部做法留到後面。行尾 `;` 結束這項顯示工作。

把 `src/main.rs` 完整改為下面內容並保存。先看入口外框，再看它唯一的一項工作：

```rust
--8<-- "examples/intro-workspace/hello/src/main.rs"
```

## 先檢查，再真的執行

在剛才的 `intro-hello` 資料夾執行：

```bash
cargo check
```

這會檢查程式是否能通過編譯，但不產生供你執行的最後執行檔，所以現在還看不到 `Hello, Rust!`。接著建置：

```bash
cargo build
```

建置會產生可執行檔，預設在 `target/debug/`。此刻仍未執行程式。最後讓 Cargo 建置需要更新的部分並執行：

```bash
cargo run
```

除了 Cargo 的進度訊息，這一次應看到：

```text
Hello, Rust!
```

修改文字、保存，再執行 `cargo run`，輸出才會跟著改變。沒有修改時，Cargo 可以沿用先前的建置成果。原始碼和執行檔是不同檔案；只修改原始碼不會讓已產生的執行檔自行變新。

## 完整的最小設定

現在打開 `Cargo.toml`。這份設定叫 **manifest**，使用 TOML（Tom's Obvious, Minimal Language）格式：`[package]` 開始一個設定區段，下面每行以 `欄位 = 值` 指定設定。

```toml
--8<-- "examples/intro-workspace/hello/Cargo.toml"
```

`name` 是 package 名稱，`version` 是你這個程式的版本，`0.1.0` 不是編譯器版本。`edition = "2024"` 選擇 Rust 2024 這組語言規則；edition 與工具的發布版本各有用途。本節的三個欄位已是完整可用的設定，不需要其他設定才能跑剛才的程式。

`cargo new` 可能另外產生空的 `[dependencies]` 區段。它用來列出需要另外取得的程式庫；這次沒有，因此留空或移除皆可。`println!` 已由 Rust 標準函式庫提供。Cargo 還會產生 `Cargo.lock`，由它記錄建置選用的 package 版本，下一節會說明怎麼使用。

## 練習：加上第二行

在第一行之後增加「這是第二行。」；先預測順序，再保存、執行。你只需要再加一項相同的顯示工作。

完整參考答案如下，仍可放進同一個 `src/main.rs`，沿用上面的完整 manifest：

```rust
--8<-- "examples/intro-workspace/hello/examples/two_lines.rs"
```

```bash
cargo run
```

```text
Hello, Rust!
這是第二行。
```

兩個 `println!` 各加一次換行。若看不到第二行，先確認有保存，再確認終端機位於剛才修改的 package。

[下一節：選對 Cargo 專案與範例](01-03-lesson.md)
