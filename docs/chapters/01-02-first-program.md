# 1.2 第一個可執行程式

這一節的任務很小：讓程式印出 `Hello, Rust!`。我們先在你自行選擇的練習資料夾建立專案，暫時不放進教材 repo。這樣第一次只需要理解一個程式、一份設定與幾個 Cargo 指令。

Cargo 管理的專案單位叫 **package**。它至少包含 `Cargo.toml` 設定檔與程式原始碼；Cargo 根據設定檔知道專案名稱和要使用的語言規則。`cargo new hello-rust` 會建立名為 `hello-rust` 的資料夾與初始檔案。`cd hello-rust` 則讓接下來的指令在這個 package 裡執行：

```bash
cargo new hello-rust
cd hello-rust
```

如果已經有同名資料夾，請換一個新名稱，不要覆蓋原來的練習。打開專案時，應能看到 `Cargo.toml` 和 `src/main.rs`；`.rs` 表示 Rust 原始碼，`src` 是 source 的縮寫，這裡存放原始碼。

## 先認識程式會用到的符號

可執行 Rust 程式從 `main` 函式開始。函式是有名稱、可以執行的一段工作；這一次只有 `main`，不需要傳入資料，也不需要產生供別的函式使用的計算結果。

`fn main()` 定義這個函式：`fn` 是定義函式的關鍵字；`main` 是名稱；空的 `()` 表示這個函式沒有輸入參數。後面的 `{` 與 `}` 圍住函式的內容，執行時會依序做裡面的工作。Rust 透過括號界定內容範圍，縮排則是幫人閱讀。

要顯示固定文字，我們使用 `println!("文字");`。雙引號裡直接寫出的內容叫**字串文字**（string literal）。這裡只需要把它當成要顯示的固定文字；沒有建立存放文字的變數，也沒有對文字做查找或修改。

`println!` 接受格式字串，將它顯示在終端機，最後加上換行。名稱後面的 `!` 表示這是 **macro**：Rust 在編譯時處理的一種特殊語法呼叫。它和一般函式的語法規則可以不同；本節需要的契約就是「括號中放一個雙引號字串，再以 `;` 結束這項工作」。此處的文字不包含 `{` 或 `}`，因為這兩個符號在格式字串中另有用途，會在顯示變數時介紹。

現在把 `src/main.rs` 的內容改成：

```rust
--8<-- "examples/hello-world/src/main.rs"
```

讀程式時，先找 `main` 的外框，再看外框中的一項工作：顯示固定文字。本例用行末的分號 `;` 結束顯示工作。

## 設定檔告訴 Cargo 什麼

`Cargo.toml` 稱為 **manifest**，也就是描述這個 package 的設定。它使用 TOML（Tom's Obvious, Minimal Language）格式：`[名稱]` 開始一個設定區段，`欄位 = 值` 設定該區段的欄位。雙引號表示文字設定值。

- `[package]` 下的 `name` 是 package 名稱，`version` 是這個 package 的版本。`0.1.0` 是新專案常用的起始版本，不是 Rust 編譯器版本。
- `edition = "2024"` 選擇 Rust 2024 這組語言規則。較新的編譯器也能編譯使用較舊 edition 的專案；不要把 edition 當成每年都得修改的年份。
- `[dependencies]` 用來列出程式依賴的外部程式庫。現在沒有外部依賴，因此區段是空的。顯示文字所需的能力由 Rust 標準函式庫提供，不需另下載一個文字顯示工具。

本節的 manifest 如下。若 Cargo 自動產生了其他不影響本節的欄位，可以保留；下列欄位已足以建立這個練習：

```toml
--8<-- "examples/hello-world/fixtures/standalone-manifest.toml"
```

## 檢查、建置、執行是三件事

| 指令 | 做完後得到了什麼 |
| --- | --- |
| `cargo check` | 先檢查原始碼中的編譯問題；不產生供你執行的最後可執行檔。適合寫到一半時先找問題。 |
| `cargo build` | 檢查並建置可執行檔。預設放在 `target/debug/`，還沒有執行它。 |
| `cargo run` | 先建置需要更新的程式，再執行它。沒有修改時，Cargo 可以沿用先前的建置成果。 |

保存檔案後，在這個 package 的資料夾依序執行：

```bash
cargo check
cargo build
cargo run
```

前兩個指令的工具訊息可能包含 `Checking`、`Compiling` 或 `Finished`；第三個才會看到我們的輸出：

```text
Hello, Rust!
```

若只改檔案而沒有保存，Cargo 讀到的仍是磁碟中的舊內容。若修改後只跑 `cargo check`，也不會看到新文字，因為你尚未執行程式。這兩個分別是編輯與執行流程的問題，和計算邏輯不同。

## 自己試一個可觀察的變化

把雙引號裡的內容改成你想顯示的一句話，保存後跑 `cargo run`。再增加第二行 `println!`，預測兩行的順序後執行。兩項顯示工作按照在 `main` 中的順序進行，每項各加一次換行。

這個練習的參考程式只增加已經介紹過的顯示工作：

```rust
--8<-- "examples/hello-world/fixtures/two-lines.rs"
```

你現在已能建立、修改、檢查並執行一個獨立 package。接著要在同一份教材 repo 裡放許多練習，我們會學 Cargo 如何分辨你要執行哪一個。
