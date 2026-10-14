# 維護與 Codex Cloud 開發

## 本機與 Cloud 共用流程

```bash
bash scripts/bootstrap.sh
bash scripts/check.sh
```

bootstrap.sh 可重複執行，會檢查 uv 是否至少為 0.13.0，缺少或版本不足時安裝固定版本；再明確安裝一般 GIL 版本的 Python 3.14，並以 --locked 確保依賴設定與鎖定檔一致。check.sh 會建置文件、檢查 Rust 格式、執行 Clippy 與測試。

兩個腳本都載入 scripts/env.sh，優先使用儲存庫上一層的 .learn-rust-tools/bin，並設定 Rust 工具鏈、Python 安裝及套件快取的位置。工具目錄可用 LEARN_RUST_TOOLS_DIR 指定；已設定的 CARGO_HOME、RUSTUP_HOME、UV_CACHE_DIR 與 UV_PYTHON_INSTALL_DIR 會保留。安裝腳本中的 export 不會沿用到下一個 shell 或 Cloud 任務，因此直接執行工具時，每個新 shell 都須先 source scripts/env.sh。

Python 版本檔保留 3.14，讓 pyenv 正常選擇本機版本。腳本只在 uv 指令中加上 3.14+gil，避免既有的 free-threaded 安裝被自動選中。

## 新版 Codex Cloud

先將此專案提交並推送至 GitHub。建立 Cloud environment 時選擇該儲存庫，讓 Codex 使用以下設定：

- Install script：在儲存庫根目錄執行 bash scripts/bootstrap.sh。
- Start skill：每個新 shell 先載入 scripts/env.sh；任務開始時同步依賴並執行檢查，需要預覽時啟動 Zensical。
- 網路：選擇 Package managers，以便取得 Python、Rust 和套件依賴；額外允許 astral.sh、releases.astral.sh 和 sh.rustup.rs，供缺少 uv、uv 版本不足或缺少 rustup 時下載安裝腳本與工具。

可貼入環境設定對話的指示：

```text
請在此儲存庫根目錄執行 bash scripts/bootstrap.sh，
並以 bash scripts/check.sh 驗證 Python 3.14、Zensical 和 Rust 範例。
Start skill 請記錄：
1. 每個新 shell 直接執行工具前，先在儲存庫根目錄 source scripts/env.sh。
   不依賴 Install script 的 export 或系統預設 PATH。
2. 任務開始時執行 bash scripts/bootstrap.sh 與 bash scripts/check.sh，
   同步依賴、確認 uv 版本並驗證環境。
3. 完成變更後執行 bash scripts/check.sh。
4. 需要預覽時先 source scripts/env.sh，再執行
   uv run --locked --python '3.14+gil' zensical serve --dev-addr 0.0.0.0:8000。
```

確認設定通過後發布環境。更新工具或依賴時重新測試並 Republish；單純儲存庫更新不會自動重跑環境安裝指令。

詳見 [官方 Cloud environments 文件](https://learn.chatgpt.com/docs/environments/cloud-environments)。

## Legacy Codex Cloud

Setup script 與 Maintenance script 都可使用：

```bash
bash scripts/bootstrap.sh
```

兩者分別準備新環境與同步快取環境的依賴。詳見 [官方 Legacy 文件](https://learn.chatgpt.com/docs/environments/cloud-environment)。

依賴與工具鏈準備完成後，可驗證不需套件下載的開發流程：

```bash
UV_OFFLINE=1 CARGO_NET_OFFLINE=true bash scripts/check.sh
```

## 文件預覽

```bash
source scripts/env.sh
uv run --locked --python '3.14+gil' zensical serve --dev-addr 0.0.0.0:8000
```

雲端的 localhost 指向雲端環境。本機的瀏覽器需要環境提供的轉送入口，或下載 CI 的 documentation-site 產物後在本機啟動靜態伺服器。

Cloud 目前列出瀏覽器操作為未支援功能；文件建置成功與 HTTP 回應可在 Cloud 驗證，視覺確認可使用本機預覽。

## 新增教學章節

1. 在 docs/ 的對應目錄加入 Markdown，並更新 zensical.toml 的 nav。
2. 可執行範例加入 examples/，並更新 Cargo.toml 的 members。
3. 文件以 snippets 引用實際原始碼，使用 rust 語法標示；引用路徑相對於專案根目錄。
4. 對預期輸出或重要行為加入有意義的測試，完成後執行 bash scripts/check.sh。

引用方式：

````markdown
```rust title="main.rs" linenums="1"
--8<-- "examples/hello-world/src/main.rs"
```
````

Zensical 建置負責文件與引用檔案的檢查；Rust 編譯與行為由 Cargo 和範例測試驗證。

## 更新依賴

```bash
source scripts/env.sh
uv add --dev 'zensical==<新版本>' --python '3.14+gil'
bash scripts/check.sh
```

提交 pyproject.toml 與 uv.lock 的變更。Rust 依賴或工具鏈更新後，同步檢查 Cargo.lock 和 rust-toolchain.toml。
