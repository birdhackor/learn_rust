# 9.1 管理共同設定與不同建置

1.3 已用 workspace 管理多個 package，7.1 也學會指定依賴版本與功能。當更多程式一起維護時，現在要控制的是：**哪些設定共用、這次編譯選哪些功能，以及使用哪一組編譯設定？**

我們繼續用小數字，避免把工具設定與新的資料處理混在一起。這次只算商品小計：單價 80、數量 2 得到 160；加上另一筆單價 120、數量 1，兩筆小計是 280。運費仍屬第一章原來的計價工作，這個示範不修改它。

本節 package 是 `ch09-engineering`，位於 `examples/ch09-engineering`。所有 Cargo 命令從 repo 根目錄執行。

## 共同設定不等於自動相依

第一章的 `version.workspace = true` 等欄位，表示成員採用 `[workspace.package]` 的共同值。依賴也能用同樣的安排：**根目錄的 `[workspace.dependencies]` 定義可供成員繼承的 dependency 設定；成員還要明確選擇採用。**

本例在根設定放 `ch01-basics` 的本機來源。這裡的 `path` 以 workspace 根目錄為起點，因此是 `examples/ch01-basics`，和 7.1 在成員 manifest 寫的 `../../ch01-basics` 有不同起點。

設定 root 之後，成員在 `[dependencies]` 寫 `ch01-basics.workspace = true`，才表示它依賴這個 package，並使用共同來源。其他沒有寫此項的成員仍不能直接呼叫它。共同設定讓多個成員可在一處維護相同來源或版本；各成員使用哪些依賴仍須明確列出。

### profile 控制編譯設定

**profile** 是一組編譯設定。平常 `cargo build`／`run` 使用 `dev`；加上 `--release` 則使用 `release`，不是切換 Rust edition。

`opt-level` 是最佳化程度，常用數字為 0 到 3。預設 `dev` 使用 0，讓反覆編譯通常花較少時間；預設 `release` 使用 3，讓編譯器投入較多最佳化工作。程式可能更快，也可能沒有明顯差異，要對相同工作實測，不能只由設定推定效能。

本 repo 的 release 設定另寫 `overflow-checks = true`，要求執行時檢查整數運算是否超出型別範圍；超出時 panic。這延續第一章的有限整數概念，也避免開發與 release 建置因預設檢查不同而掩蓋錯誤。檢查不會自動把大數字變成可表示的值；程式仍要遵守其輸入與結果範圍。

profile 設定寫在 **workspace 根 manifest**；放進某個成員 manifest 的同名設定會被 Cargo 忽略。下面是根設定的本節節錄，省略其他成員與欄位，不要拿它覆蓋整份 repo 設定：

```toml
--8<-- "examples/ch09-engineering/fixtures/workspace-excerpt.toml"
```

`ch09-engineering` 的成員設定則延用第一章的 package 繼承，並選用共同 dependency：

```toml
--8<-- "examples/ch09-engineering/Cargo.toml:package"
```

## 先定義自己的 feature，再選擇它

7.1 的 `features = ["derive"]` 選的是 dependency 的功能。現在我們替**自己的 package** 定義名為 `batch` 的 feature，讓使用者可以選擇是否編入一個處理兩筆訂單的公開函式。

成員 manifest 的 `[features]` 用 `名稱 = [要連帶啟用的其他功能]` 定義開關。`batch = []` 表示有這個 feature，但它不連帶啟用其他項目；`default = []` 表示本 package 預設不開任何列在這裡的功能。

```toml
--8<-- "examples/ch09-engineering/Cargo.toml:features"
```

`cargo ... --features batch` 選擇本 package 的功能，和 7.1 的 `cargo add ... --features derive` 修改 dependency 設定是兩項不同工作。`--no-default-features` 不啟用本 package 的 `default` 列表；本例列表原本就為空，因此此旗標不會另改結果。`--all-features` 啟用本 package 的全部 features，本例也就是 `batch`。

features 通常用來增加可選能力。這裡保留單筆函式及其結果，再增加批次函式；沒有用 feature 偷偷替原函式換一套計價規則。多個依賴使用者要求某個 library 的不同 features 時，Cargo 通常會把需要的功能合在一起；因此功能最好能同時啟用，而非互相排斥。

## cfg 決定哪些項目進入這次編譯

1.10 的 `#[cfg(test)]` 讓同檔測試只在測試建置中出現。**條件編譯**就是按建置條件保留或排除程式項目，不必等到程式執行時才選擇。

`#[cfg(feature = "batch")]` 是相同形式：當本 crate 啟用 `batch` 時，保留緊接在後面的函式；未啟用時，該函式根本不在這次編譯的公開介面中。雙引號的 `batch` 必須和 manifest 定義的 feature 名稱相符。

先看 library。`order_subtotal` 一直保留，使用已教的第一章計算；`two_orders` 接受兩個 `[i32; 2]`，每筆依序放單價與數量，使用各筆資料計算後相加。這個附加函式才受 feature 控制：

```rust
--8<-- "examples/ch09-engineering/src/lib.rs:library"
```

### cfg! 的結果是 bool

`cfg!(feature = "batch")` 是另一個 macro，按同一建置條件產生 `true` 或 `false`，可以保存到變數或顯示。它**不會排除包住它的程式**；例如放在普通 `if` 條件時，兩個 branch 的 Rust 程式仍需能通過編譯檢查。

呼叫可選函式時，我們使用 `#[cfg(...)]` 包住整個呼叫 block，包括所需的 `use`。本節也使用兩個平台條件：`unix` 表示編譯目標屬於 Unix 類系統，例如 Linux 與 macOS；`target_os = "windows"` 表示目標作業系統是 Windows。它們只按建置目標產生 bool。

**編譯目標 target** 是這份程式準備在哪種平台執行；**host** 則是執行編譯工具的本機。沒有另外選目標時，target 通常與 host 相同。即使將來替其他平台建置，這些 crate 內的條件看的是它的 target，而不是執行到這行時重新偵測作業系統。profile 與 target 是不同選擇：`--release` 不會把 Linux 程式變成 Windows 程式。

現在看 `src/main.rs`。同樣的 `main` 可以用於有或沒有 `batch` 的建置；平台條件只提供可觀察的 bool：

```rust
--8<-- "examples/ch09-engineering/src/main.rs"
```

從 repo 根目錄分別執行，`--features` 已在前面介紹：

```bash
cargo run -p ch09-engineering --locked
cargo run -p ch09-engineering --features batch --locked
```

未啟用時，第一行是 `單筆商品小計=160，batch=false`，沒有兩筆小計那一行。啟用後第一行的 bool 成為 `true`，接著顯示 `兩筆商品小計=280`。兩次最後都顯示編譯目標條件；在 Linux 示範環境中是 `unix=true，windows=false`，其他平台應以自己的 target 為準。

### 為什麼不能只寫普通 if

下面是故意失敗的 fixture，沿用相同兩筆資料，但把呼叫改用普通 `if cfg!(...)`。沒有啟用 `batch` 時，函式已被 attribute 排除，`if` 的 body 卻仍然要接受型別與名稱檢查：

```rust
--8<-- "examples/ch09-engineering/fixtures/compile-fail/cfg_bool_is_not_removal.rs"
```

因此會得到 `cannot find function two_orders`：`false` 只讓執行時不進入 branch，沒有讓裡面不存在的名稱變成合法。前一份正確版本用 attribute 排除整個呼叫 block，才讓未啟用功能的建置也成立。

## 檢查功能組合，再使用 release 建置

功能控制的不只使用者程式，測試也需要在相同條件下呼叫可選介面。這份 integration test 一直驗證單筆結果，再於 `batch` 啟用時確認兩筆各自的單價與數量；數量零也能看出兩筆沒有誤用相同資料：

```rust
--8<-- "examples/ch09-engineering/tests/features.rs"
```

下面的命令分別檢查預設與全部功能。預設應有一項 integration test，全部功能應有兩項；`--all-targets` 仍是第一章的 library／binary／範例／測試等目標，不是指所有作業系統：

```bash
cargo test -p ch09-engineering --locked
cargo test -p ch09-engineering --all-features --locked
cargo clippy -p ch09-engineering --all-targets --all-features --locked
cargo build -p ch09-engineering --features batch --release --locked
cargo run -p ch09-engineering --features batch --release --locked
```

release 程式的商品小計仍是 160 與 280。Cargo 的建置訊息會改成 `release`，產物預設在共同 `target/release/`；原始碼與輸入相同，這裡只驗證設定能建置、結果相符，沒有作速度測量。

## 練習：設定放在哪裡，改變什麼

先自己回答：只把依賴放進 `[workspace.dependencies]`，某個成員就能呼叫它嗎？關掉 `batch` 會改掉單筆 160 的答案嗎？`cfg!` 回傳 false 就能遮住未定義的函式呼叫嗎？

答案依序是：成員仍須明確選用；單筆介面與答案保留，只移除批次介面；不能，普通 branch 仍需通過編譯。再執行 `cargo run -p ch09-engineering --no-default-features --locked`，確認本例空的預設列表使結果與第一次相同。

你已能把共同依賴、可選介面、編譯設定與平台條件分開判斷。下一節會讓 Cargo 在主要 crate 編譯前先執行一個 Rust 程式，處理建置所需的資料；它仍遵守已建立的工具與輸入／輸出契約。
