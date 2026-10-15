# 1.3 選對 Cargo 的 package、crate 與 workspace

教材會保存許多不同階段的程式。我們需要能指定「今天只跑這一個」，而且回頭看第一個程式時，它仍保持只有已學過的內容。這一節改用教材保存的檔案。若你還沒有取得教材，在瀏覽器打開 [learn_rust 的 GitHub 頁面](https://github.com/birdhackor/learn_rust)，按 **Code → Download ZIP**，再解壓縮。不需要先會使用 Git。

解壓後的資料夾通常叫 `learn_rust-main`。先在它的上層資料夾開啟終端機，再切進去：

```bash
cd learn_rust-main
```

確認這裡同時有 `Cargo.toml`、`docs` 與 `examples`，這就是**教材 repo 的根目錄**。repo 是 repository 的簡稱，這裡指整份教材的檔案集合；若解壓時改了資料夾名稱，`cd` 後面也用你實際的名稱。以下路徑都從這裡開始。

## 一個設定檔與一個編譯單位

上一節建立的 `intro-hello` 是一個 **package**：Cargo 以一份 `Cargo.toml` 管理它的名稱、版本與程式。**crate** 則是 Rust 編譯器一次處理的一個編譯單位。現在的 `src/main.rs` 是一個 binary crate 的起點，編譯後能執行；一個 package 也能包含供其他程式使用的 library crate，但本章先不建立它。

教材把第一個 package 放在 `examples/intro-workspace/hello/`。其完整設定與入口如下，與上一節相同：

```toml
--8<-- "examples/intro-workspace/hello/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/hello/src/main.rs"
```

`--manifest-path` 指定要讀哪一份 manifest。先用它從 repo 根目錄找到這個 package：

```bash
cargo run --manifest-path examples/intro-workspace/hello/Cargo.toml
```

輸出仍是 `Hello, Rust!`。參數中的路徑是一份檔案的位置；package 的名稱則是檔案裡的 `intro-hello`，兩者不必相同。

## 用 workspace 一起管理幾個 package

現在加入第二個只印文字的 package。**workspace** 是共同管理的一組 package；成員共用一份 `Cargo.lock` 與建置輸出的 `target` 資料夾。它不會把所有程式接成一個可執行檔。

下面是教材入門 workspace 的**完整設定**，檔案在 `examples/intro-workspace/Cargo.toml`：

```toml
--8<-- "examples/intro-workspace/Cargo.toml"
```

`[workspace]` 宣告這是 workspace 設定。`members` 的中括號是一份設定值清單；其中的文字是相對於這份 manifest 的成員資料夾路徑。各資料夾都放自己的 `Cargo.toml`。`resolver` 指定 Cargo 選擇依賴版本時採用哪一版規則；這裡明寫 `"3"`，與本書的 edition 2024 搭配。本章沒有外部依賴，先保留這個完整設定即可。

第二個成員 `greeting/` 使用下列完整 manifest 與程式：

```toml
--8<-- "examples/intro-workspace/greeting/Cargo.toml"
```

```rust
--8<-- "examples/intro-workspace/greeting/src/main.rs"
```

現在 manifest 指向整個 workspace，`-p` 再用 package 名稱選成員：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-greeting
```

輸出是 `你好，Rust！`。把 `-p` 後面改成 `intro-hello` 就會選回第一個程式。若 workspace 有多個可執行程式，只說「執行 workspace」無法表達你想看哪個輸出，因此本書明確指定。

## 同一 package 中的獨立範例

`hello/examples/two_lines.rs` 保存上一節的第二個階段。Cargo 把 `examples/` 內的這種程式視為可選的**範例 target**；target 是你要求 Cargo 建置的特定成果。每個範例都有自己的 `main`，是獨立的 binary crate。

```rust
--8<-- "examples/intro-workspace/hello/examples/two_lines.rs"
```

`--example` 使用檔名去掉 `.rs` 後的名稱選它：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-hello --example two_lines
```

現在顯示兩行；沒有 `--example` 時，這個 package 仍執行 `src/main.rs` 的一行版本。這讓階段快照可以並存。

之後本章的數字練習放在成員 `steps/`，其名稱與完整設定是：

```toml
--8<-- "examples/intro-workspace/steps/Cargo.toml"
```

它的預設入口仍只有已教過的文字顯示：

```rust
--8<-- "examples/intro-workspace/steps/src/main.rs"
```

下一節才選 `steps/examples/` 裡的數字範例。上面介紹的這幾個 package 都沒有 `src/lib.rs`，選擇一個範例時，也不會連帶編譯一份包含後續概念的 library。

## Cargo.lock 與 --locked

`Cargo.toml` 表達你要求的設定；`Cargo.lock` 記錄 Cargo 已選定的確切 package 版本。這個 workspace 也保存了一份 `Cargo.lock`，由 Cargo 產生並管理，不必手動修改其中的格式。新增或調整 package 時，Cargo 會按設定重新記錄選擇結果。

教材已保存它，因此後續指令加上 `--locked`：要求沿用鎖定結果，若必須更新鎖定檔才能繼續，指令就失敗，提醒你設定與保存的狀態不一致。這個選項**不會**阻止你修改 `.rs`；修改程式後仍可重新建置。

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-hello --example two_lines --locked
```

到這裡，每個選項都各有工作：路徑找 workspace、`-p` 選成員、`--example` 選範例、`--locked` 檢查版本鎖定。本書 repo 還有較大的維護用 workspace；本章指令都明確指向上面這一份入門 workspace。

## 練習與完整答案

請選 `intro-greeting` 的 `two_lines` 範例，預期依序印出「你好，Rust！」與「開始今天的練習。」。這份完整程式如下，設定沿用同頁的 `greeting/Cargo.toml`：

```rust
--8<-- "examples/intro-workspace/greeting/examples/two_lines.rs"
```

參考指令：

```bash
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-greeting --example two_lines --locked
```

兩個 package 的範例都叫 `two_lines`，但 `-p` 已選定成員，因此 Cargo 知道你要哪一份。

[下一節：型別與數值運算](01-04-lesson.md)
