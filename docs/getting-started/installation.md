# 準備開發環境

## 學習目標

- 認識 rustup、rustc 與 Cargo 的用途。
- 確認自己的環境可以編譯並執行第一個範例。

## 安裝 Rust

依照 [Rust 官方安裝指南](https://www.rust-lang.org/tools/install) 安裝 rustup。

rustup 管理 Rust 工具鏈；rustc 負責編譯程式；Cargo 管理專案、依賴與建置。

確認指令可使用：

```bash
rustup --version
rustc --version
cargo --version
```

## 執行教材範例

下載此教材的儲存庫後，在根目錄執行：

```bash
cargo run -p hello-world --locked
```

rustup 會依 rust-toolchain.toml 使用教材固定的工具鏈版本。Cargo 會編譯並執行 hello-world 範例。

!!! tip "學習 Rust 的起點"

    執行 Rust 範例需要 Rust 工具鏈。Python 與 uv 用於維護和預覽本站；相關流程在儲存庫的 README.md。

接著閱讀 [第一個 Rust 程式](hello-world.md)。
