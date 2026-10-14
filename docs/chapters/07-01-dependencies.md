# 7.1 讓 Cargo 準備外部程式庫

前面的小程式使用標準函式庫，以及我們自己寫的 library。接下來要把資料處理接上檔案與命令列；有些工作可以使用別人維護的程式庫。這一節先處理一個具體問題：**如何讓 Cargo 知道使用哪個程式庫、哪些功能，以及哪一組版本？**

本節在 repo 根目錄操作 `ch07-tooling` package，位於 `examples/ch07-cli/tooling`。它延用第一章的數字計價函式；本頁先準備工具設定，命令列參數解析的 Rust 介面會在 7.6 教清楚後才呼叫。

## registry、package 與使用它的 crate

**dependency** 是目前 package 需要的另一個 package。Cargo 先找到它的來源與合適版本，再建置需要的 library crate，讓我們的程式能呼叫公開功能。加入 dependency 不代表這個功能已經執行過。

[crates.io](https://crates.io/) 是 Cargo 預設使用的 **registry**：保存已發布 package 與版本資訊的註冊網站。它讓 Cargo 能依名稱下載程式庫及其必要依賴。第一章使用的 `std` 是 Rust 標準函式庫，由工具鏈提供；不需另外在 manifest 加一個名叫 `std` 的 dependency。

[docs.rs](https://docs.rs/) 則提供這些 library 的 **API** 文件。API 是讓其他程式使用的公開介面，例如已經學過的公開函式。crates.io 回答「有哪些版本可取得」，docs.rs 回答「這個版本提供什麼公開功能，以及怎麼使用」。

我們會用到 `clap`，它的工作是解讀命令列參數；`tempfile` 則為後面的檔案測試準備暫存資料。現在只設定它們的來源與功能，不先塞入尚未介紹的呼叫方式。手上的數字程式仍使用第一章已教的函式。

### 名稱出現在哪裡

manifest 裡記錄的是 dependency 名稱；Rust 程式則使用 library crate 的路徑。1.9 已看過 `ch01-basics` 在 Rust 中寫成 `ch01_basics`。同樣地，加入一個 package 後，仍要依它提供的 crate 名稱與公開項目來寫 `use`，不能把網站網址當成 Rust 路徑。

本例把 `ch01-basics` 當作另一個 package 使用。它位於本機 repo，所以用 `path` 指定來源；這條 path 以**宣告 dependency 的成員 manifest 所在資料夾**為起點。從 `examples/ch07-cli/tooling` 先往上兩層，再走到 `ch01-basics`，因此寫成 `../../ch01-basics`。`..` 表示上一層資料夾。

## 讀懂 manifest 中的三個選擇

以下設定使用 TOML 的**行內表格**：`{ 欄位 = 值, 欄位 = 值 }` 把同一個 dependency 的數個設定放在一起。

- `[dependencies]` 放 library、binary 等一般程式需要的依賴。`path` 指向本機 package；`version` 則給 registry 的版本要求。沒有 `path` 等來源設定時，Cargo 使用預設 registry。
- `[dev-dependencies]` 放測試或範例等開發工作需要的依賴；它不會只因為寫在這裡，就成為一般 library 程式可用的依賴。本例先準備測試之後會需要的 `tempfile`。
- **feature** 是程式庫提供的具名功能開關。本例的 `features = ["derive"]` 請求 `clap` 提供名為 `derive` 的功能；7.6 的宣告式參數設定方式需要它。這個名稱屬於 `clap`，不表示我們自己的 package 同時多出同名 feature。

`clap` 預設啟用一組常用功能，本例保留它們，再加上 `derive`。若需要關掉某個 dependency 的預設功能，可在同一張行內表格寫 `default-features = false`，再列需要的 features。是否能這樣使用，要依該 library 的文件確認；不能猜每個程式庫都有相同開關。

還要注意版本設定的含義。`4.6.7` 三段分別叫 major、minor、patch：發布者以第一段表示不相容的公開介面改動，後兩段表示相容的功能新增與修正。這是版本約定，並非編譯器對程式庫的保證。

對 major 大於零的版本，Cargo 預設把 `version = "4.6.7"` 當作從 4.6.7 起、尚未進入 5.0.0 的相容版本要求；**不是精確鎖在 4.6.7**。若要在 manifest 限定某一版本，`version = "=4.6.7"` 才是精確要求。本例讓 lockfile 保存實際使用版本。

`ch07-tooling` 的依賴設定是：

```toml
--8<-- "examples/ch07-cli/tooling/Cargo.toml:dependencies"
```

你可以在 [clap 的 docs.rs 頁面](https://docs.rs/clap/4.6.7/clap/) 找到目前使用的版本，再打開 Features 查看功能清單。初次閱讀先核對「版本是否相同」「需要哪個 feature」；等學會特定介面，再閱讀它的參數、回傳值與錯誤。第三方文件的完整範例不一定遵守本書順序，不直接搬來當本節程式。

### 確認本機 dependency 真正被使用

`src/main.rs` 直接呼叫第一章的 `total_with_shipping`：兩筆單價分別為 80 與 120、各買一件，含運費各為 100 與 140，相加得到 240。跨 package 呼叫仍用 1.9 已教的 crate 路徑與 `use`，新增的是 manifest 宣告相依關係：

```rust
--8<-- "examples/ch07-cli/tooling/src/main.rs"
```

即使兩個 package 都在同一 workspace，也不能省略 `ch01-basics` 的 dependency 設定。workspace 管理它們，manifest 的依賴項目才告訴 Cargo 哪個 package 要使用哪個 library。

## 用 cargo add 修改設定

`cargo add` 把指定依賴加入 manifest，也更新需要的版本解析紀錄。名稱後的 `@版本` 指定版本要求；`--features derive` 加上具名功能，`--dev` 改放入開發依賴區段。`-p` 仍像第一章一樣選 package。

以下是在**練習副本**重建上述兩個 registry 設定的命令；repo 已有這些設定時，會更新現有項目，不會另建立第二個 `clap`：

```bash
cargo add clap@4.6.7 --features derive -p ch07-tooling
cargo add tempfile@3.27.0 --dev -p ch07-tooling
```

這兩個命令是刻意修改 dependency 設定的工作，因此不要求保留舊 lockfile。完成後，才用 `--locked` 確認 manifest 與解析紀錄相符：

```bash
cargo check -p ch07-tooling --locked
cargo run -p ch07-tooling --locked
```

第一次可能看到下載與編譯依賴的訊息。數字程式最後輸出：

```text
兩筆訂單總額=240
```

能建置表示 Cargo 已準備好這組設定。這還沒有驗證 `clap` 的參數解析行為；那要等真正使用它，並用對應測試確認。

## lockfile 保存哪一組實際版本

`Cargo.lock` 保存這次解析到的完整 dependency 組合，包括依賴的依賴。未修改設定時，下一次正常建置會沿用紀錄，而不是每次都挑新的版本。`--locked` 再要求這次操作不能更動紀錄。

`cargo update` 則刻意重新選擇符合 manifest 要求的版本。後面直接接**要更新的 dependency package**，和 `cargo run -p ch07-tooling` 選要執行的 package 是不同工作。例如下面命令只在練習副本嘗試更新 `clap` 這個 dependency；它仍可能連帶改動與之相關的依賴：

```bash
cargo update clap
```

更新後先檢查 manifest 與 lockfile 的差異，再重新執行程式與測試。不要為了讓 `--locked` 過關就刪掉 lockfile；先判斷你是否真的要改依賴。lockfile 也不保存作業系統與所有編譯環境，不能把它當作跨平台結果完全相同的保證。

## 練習：哪個檔案回答哪個問題

在練習副本查 `clap` 的三項資訊：manifest 要求什麼版本範圍、要求哪個 feature、lockfile 實際解析到什麼版本。答案前兩項是「4.6.7 起至 5.0.0 之前」與 `derive`；第三項要讀你副本中對應 `clap` 項目的 `version`，不能只照抄 manifest。

再說明為什麼 `tempfile` 出現在另一個區段：它準備給測試使用，並非目前數字 library 的運算材料。下一節會沿用這個小 library，讓公開介面的使用方式同時成為文件與可執行檢查。
