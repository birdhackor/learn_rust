# 10.1 用 Cargo 宣告依賴、版本與 features

前面的程式使用標準函式庫與自己寫的 library。接下來要讀檔案、處理命令列參數，也會使用別人維護的程式庫。這一節先回答：**Cargo 如何知道到哪裡找程式庫、需要哪些功能，以及使用哪一組版本？**

本節的 package 是 `intro-cargo-setup`，位於 `examples/intro-workspace/cargo-setup/`。它只有一個數字程式，會使用第 2.6 節已完成的計價 library；本節不改變運費規則。

## dependency 指向另一個 package

**dependency** 是目前 package 所需要的另一個 package。Cargo 依 manifest 找到它、準備所需的 library crate，讓我們的程式能使用其公開項目。加入依賴本身不會執行那個 library 的功能。

先使用本機的 `intro-modules-basic`。它與本節 package 都在 `intro-workspace/` 下，因此從 `cargo-setup/Cargo.toml` 所在資料夾出發，先以 `..` 往上一層，再進入 `modules-basic/`。依賴的 `path` 寫成 `../modules-basic`；起點是**宣告依賴的 manifest 所在資料夾**，不是執行命令時的所在資料夾。

`[dependencies]` 是放一般程式依賴的 TOML 區段。

即使兩個 package 已列在同一 workspace，呼叫者仍須宣告這項 dependency。workspace 管理成員，dependency 則表達「這個 package 要使用另一個 package 的 library」。

## 從 crates.io 取得已發布的 library

[crates.io](https://crates.io/) 是 Cargo 預設使用的 **registry**，保存已發布 package 的名稱、版本與下載資訊。沒有指定 `path` 等其他來源時，Cargo 依名稱與版本要求從這個 registry 找依賴。

`std` 是工具鏈提供的標準函式庫，不必在 manifest 加入名為 `std` 的 dependency。接下來會設定的 `clap` 與 `tempfile` 則是第三方 package：

- `clap` 用來解讀命令列參數，第 10.6 節才會使用它的 Rust 介面。
- `tempfile` 用來準備測試中的暫存檔案與資料夾，第 10.8 節才會呼叫它。

本節先準備設定；數字程式只呼叫已教過的計價函式。第三方程式庫內部如何實作，不是這份數字程式需要自行重寫的工作。

## 同一份設定同時選來源、版本與功能

**feature** 是 library 提供的具名功能開關。`clap` 的 `derive` 功能支援後面要學的宣告式參數設定，所以本例明確請求 `features = ["derive"]`。這個功能名稱屬於 `clap`，不會替我們自己的 package 定義一個同名 feature。

`clap` 已有預設啟用的功能，本例保留它們，再加上 `derive`。若某個 library 的文件要求關閉預設功能，可在它的行內表格加入 `default-features = false`，再列出需要的 features。每個 library 的開關與相依規則不同，要查對應版本的文件，不能只依名稱猜測。

`[dev-dependencies]` 放測試、範例等開發工作需要的依賴。本例將 `tempfile` 放在這裡；一般 library 或 binary 不會因此就能把它當成一般依賴使用。

以下是本節**完整** `cargo-setup/Cargo.toml`。前三個欄位仍是第一章已教的 package 名稱、版本與 edition。依賴有不只一項設定時，用 TOML 的**行內表格** `{ 欄位 = 值, 欄位 = 值 }` 放在同一組；下面的本機依賴與 `clap` 設定就是這種形式：

```toml
--8<-- "examples/intro-workspace/cargo-setup/Cargo.toml"
```

### 版本要求與精確版本不同

`4.6.7` 的三段稱為 major、minor、patch。**Semantic Versioning**，簡稱 **SemVer**，是發布者表達版本相容性的約定：對已進入 1.0 的版本，major 改動表示不相容的公開介面變更，minor 與 patch 分別用於相容的功能新增與修正。這是發布約定，並非編譯器保證 library 沒有行為問題。

對本例 major 大於零的版本，`version = "4.6.7"` 表示允許從 4.6.7 起、尚未進入 5.0.0 的相容版本；它沒有精確鎖在 4.6.7。同理，`tempfile = "3.27.0"` 允許從 3.27.0 起至 4.0.0 之前的版本。

若真的要在 manifest 寫精確要求，字串才改成 `"=4.6.7"`。本頁實際 manifest 保留相容版本要求，由 lockfile 保存這次真正選到的版本。0.x 的相容範圍有另一套界線，這裡不把上述 major 規則直接套過去。

[docs.rs](https://docs.rs/) 提供 library 的 **API（Application Programming Interface，應用程式介面）** 文件。API 是給其他程式使用的公開介面，例如公開函式、型別與方法。crates.io 回答「有哪些版本可取得」，docs.rs 回答「這個版本提供什麼公開功能、如何使用」。例如 [clap 4.6.7 的功能清單](https://docs.rs/crate/clap/4.6.7/features) 可核對預設功能與 `derive`；呼叫特定 API 前，還要閱讀它的輸入、輸出與使用條件，下一節會實作一份這樣的文件。

## 使用本機依賴，看到同樣的數字

在 Rust 程式中，package 的連字號換成 library crate 名稱的底線，這是第 2.6 節已用過的規則。以下完整 `cargo-setup/src/main.rs` 從 `intro_modules_basic::pricing` 呼叫公開函式：

```rust
--8<-- "examples/intro-workspace/cargo-setup/src/main.rs"
```

80 元與 120 元的商品分成兩筆訂單、各買一件，各加 20 元運費，總額是 100 + 140 = 240。跨 package 呼叫使用的是 Rust 的 `::` 名稱路徑；manifest 的 `path` 則是找資料夾，兩者的角色不同。

在 repo 根目錄執行：

```bash
cargo check --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-setup --locked
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-setup --locked
```

第一次可能先下載與編譯依賴，主要程式最後顯示：

```text
兩筆訂單總額=240
```

這確認本機 library 的依賴與呼叫能連起來；此程式尚未呼叫 `clap`，因此不能拿這個結果當成參數解析行為的檢查。

## 用 cargo add 修改設定

`cargo add` 將依賴加入 manifest，並更新需要的版本解析紀錄。名稱後的 `@版本` 指定版本要求，`--features derive` 加上具名功能，`--dev` 選開發依賴區段；`--manifest-path` 與 `-p` 仍明確選擇入門 workspace 與本節 package。

在**自己的練習副本**可以用下列指令重建本頁兩個 registry 設定。已有同名依賴時，它會更新現有項目，不會新增第二個 `clap`：

```bash
cargo add --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-setup clap@4.6.7 --features derive
cargo add --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-setup tempfile@3.27.0 --dev
```

這是刻意修改依賴設定的工作，不要求維持原 lockfile。完成後，再用前面的 `cargo check ... --locked` 確認 manifest 與解析紀錄相符。

## lockfile 與依賴樹回答不同問題

入門 workspace 共用 `examples/intro-workspace/Cargo.lock`。它保存本次解析到的實際版本組合，也包含「依賴的依賴」。未修改設定時，後續建置會沿用紀錄；`--locked` 再要求此次操作不能修改它。通常由 Cargo 維護，不手動填一份依賴清單。

想看某個 package 連到哪些依賴，可以用 `cargo tree`。`--depth 1` 將顯示深度限制為一層，先只看直接依賴：

```bash
cargo tree --manifest-path examples/intro-workspace/Cargo.toml -p intro-cargo-setup --depth 1 --locked
```

先找樹根的 `intro-cargo-setup`，下面可看到 `clap` 與 `intro-modules-basic`，以及開發依賴區段的 `tempfile`。每個名稱旁的版本是實際解析版本。移除 `--depth 1` 會展開更多層；若出現 `(*)`，表示那個 package 的後續依賴已在別處列過，此處省略重複內容。這是閱讀依賴關係的工具，不能取代程式行為的測試。

`cargo update` 則刻意重新選擇符合 manifest 要求的版本。下列指令只在練習副本嘗試更新 `clap`，仍可能連帶改動相關依賴：

```bash
cargo update --manifest-path examples/intro-workspace/Cargo.toml clap
```

這裡的 `clap` 是**要更新的依賴 package**，不同於 `cargo run -p intro-cargo-setup` 選要執行的 package。更新後檢查 manifest、lockfile 的差異，再執行適當程式與測試。不要為了讓 `--locked` 過關就刪除 lockfile；先判斷這次是否真的要改依賴。

lockfile 沒有保存作業系統與所有編譯環境，因此也不是跨平台結果完全相同的保證。

## 本機依賴的完整來源

本頁真正使用的是第 2.6 節保留的最小 library，不會連帶載入本書後續章節。以下三份檔案就是這項本機依賴的設定與 library 來源；它的 binary 與其他範例沒有被本節指令選中。

??? example "展開 modules-basic 的 manifest 與 library 全部來源"

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

## 練習與完整答案：設定與解析各看哪裡

在練習副本查出 `clap` 的版本要求、要求的 feature、lockfile 的實際版本。再說明為何 `tempfile` 放在另一個區段，以及哪個設定讓本節數字程式找到本機計價 library。

完整設定答案就是以下 manifest；保留完整欄位，方便逐項對照：

```toml
--8<-- "examples/intro-workspace/cargo-setup/Cargo.toml"
```

`clap` 的要求是 4.6.7 起至 5.0.0 之前，額外要求 `derive`，並保留預設功能。實際版本要讀你副本 lockfile 中 `clap` 項目的 `version`，也可看 `cargo tree` 的對應列；不能只照抄 manifest。

`tempfile` 是預備給測試的開發依賴，因此放在 `[dev-dependencies]`。本機 library 則由 `[dependencies]` 中的 `intro-modules-basic` 與相對 `path` 找到。可執行程式與完整本機依賴都在本頁；兩筆訂單結果仍是 240。

[下一節：閱讀 API 文件與測試公開介面](10-02-lesson.md)
