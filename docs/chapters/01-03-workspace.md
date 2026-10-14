# 1.3 在教材 workspace 選對 package

上一節只有一個 package，Cargo 自然知道要建置哪個程式。這份教材則會放很多可執行範例：第一個印文字，後面的練習處理數字，再後面處理資料與檔案。現在需要回答的是：**當所有範例放在同一份 repo，怎麼指定這次要跑哪一個？**

## package 與 crate 分別指什麼

package 是由一份 `Cargo.toml` 管理的一組程式來源與設定。**crate** 則是 Rust 一次編譯的一個程式單位。一個 package 可以包含一個或多個 crate；package 和 crate 不是同義詞。

上一節的 package 有 `src/main.rs`，它是 **binary crate** 的根檔案：從這個檔案開始編譯出一個可以執行的程式，並以 `main` 作為程式入口。後面會學另一種 **library crate**，它提供其他程式呼叫的功能，而不是以自己的 `main` 開始執行。這一節只執行第一種。

**workspace** 把多個 package 收在一起，讓它們共用版本設定、依賴版本紀錄與建置成果的位置。每個 package 仍有自己的名稱和原始碼；workspace 沒有把它們合成一個 `main`。

在教材中，檔案的位置如下；圖中的 `hello-world` 是 repo 已經提供的 package，和上一節自行建立的 `hello-rust` 是兩個不同的練習資料夾：

```text
learn_rust/
  Cargo.toml                   workspace 設定
  Cargo.lock                   共同的依賴版本紀錄
  rust-toolchain.toml           教材固定的工具版本
  examples/
    hello-world/
      Cargo.toml               hello-world package 設定
      src/
        main.rs                binary crate 的根檔案
```

## 打開 repo 並確認目前的位置

你可以從 [GitHub repo](https://github.com/birdhackor/learn_rust) 選 Code → Download ZIP，解壓縮後以終端機進入資料夾。若使用版本控制工具 Git，`git clone` 會下載 repo 並保留更新所需的資料；下載方式不影響後面的 Rust 程式。

確認目前資料夾直接包含根目錄的 `Cargo.toml`、`Cargo.lock` 和 `examples`。若下載 ZIP 後資料夾叫 `learn_rust-main`，請以你的實際資料夾名稱為準。下文說的 **repo 根目錄** 就是這一層，不是 `examples/hello-world/src`。

repo 的 `rust-toolchain.toml` 請求 Rust 1.99.0 與相關工具。已安裝 `rustup` 時，在這個 repo 執行 Rust 指令會自動選用該工具鏈；首次可能需要連線下載。這讓大家使用同一版本驗證，而不影響其他資料夾的練習。

## 根設定與成員設定如何配合

根 manifest 的結構如下。這是一份**節錄模型**，只列出現在要跑的 `hello-world` 成員；實際 repo 的 `members` 還會列出其他章節的 package。不要用這份節錄覆蓋 repo 的完整設定。

`[workspace]` 宣告這份 manifest 管理的是 workspace。`members` 是成員資料夾路徑的清單，路徑以這份根設定所在的資料夾為起點；因此 `examples/hello-world` 指向圖中的 package。方括號 `[...]` 包住清單項目。

`resolver = "3"` 選擇 Cargo 的依賴解析規則版本 3，與本書 edition 2024 的配置搭配使用。依賴解析的任務，是在各 package 請求依賴時決定能共同使用哪些版本；眼前沒有外部依賴，不需手算這個過程，但不應把這個 `3` 誤認為成員數或 Rust 版本。

`[workspace.package]` 設定成員可以採用的共同欄位。`version` 和 `edition` 已在上一節介紹；新增的 `rust-version = "1.99"` 表示這個 package 宣告至少需要這個 Rust 編譯器版本。`publish = false` 禁止透過 Cargo 把教材 package 發布到 crates.io 這個公開程式庫註冊網站，與能否在本機執行無關。TOML 的 `true`／`false` 是是／否設定值。

```toml
--8<-- "examples/hello-world/fixtures/workspace-excerpt.toml"
```

共同欄位需要由成員明確選擇採用。

`name` 留在成員中，因為每個 package 都要有自己的名字。`edition.workspace = true` 的意思是「這個成員的 edition 使用 workspace.package 裡的共同值」，其他 `.workspace = true` 欄位也是同樣的繼承方式。它不是把成員的 edition 設成布林值。

`examples/hello-world/Cargo.toml` 因而寫成：

```toml
--8<-- "examples/hello-world/Cargo.toml"
```

## 選 package 與保留鎖定版本

接下來會用兩個 Cargo 選項。`-p hello-world` 指定 package，`hello-world` 必須符合成員 manifest 的 `name`。它不是檔案路徑，因此不寫成 `-p examples/hello-world`。

`Cargo.lock` 由 Cargo 管理，保存實際選用的 package／依賴版本。workspace 的成員共用根目錄的 lockfile。`--locked` 表示這次操作必須使用已保存的版本紀錄，不允許悄悄修改它；若設定的依賴與紀錄不一致，Cargo 會停下來，讓差異被明確處理。它不表示「不連線」，也不是防止修改 Rust 原始碼的鎖。

在 repo 中執行單一範例的編譯檢查、建置、執行與測試時，本書會寫出 `-p` 與 `--locked`。上節新建的獨立練習第一次還沒有 lockfile，因此當時不加 `--locked`；Cargo 建立紀錄後，才有可鎖定的版本。

現在在 repo 根目錄執行：

```bash
cargo check -p hello-world --locked
cargo run -p hello-world --locked
```

第一個指令檢查指定的 package，第二個執行它；程式輸出仍是 `Hello, Rust!`。

如果你切換到 `examples/hello-world`，Cargo 也能找到上層 workspace；建置成果仍共用根目錄的 `target/`。為了讓每頁命令的起點一致，接下來都回到 repo 根目錄執行。僅在 `src` 裡改檔案，不會讓它變成另一個 package。

## 檢查你選的是什麼

試著用一句話說出剛才指令的四個部分：Cargo 執行 `run` 工作，選名稱為 `hello-world` 的 package，要求保留已鎖定的依賴版本，最後執行它的 binary crate。再指出根設定與成員設定各在哪一層。

你已能把「我要跑哪個練習」說成明確的命令。下一節會開始計算數字，使用同一 workspace 中名為 `ch01-basics` 的 package；我們將用變數保存計算材料，而不再只印固定文字。
