# 維護與 Codex Cloud 開發

## 本機與 Cloud 共用流程

```bash
bash scripts/bootstrap.sh
bash scripts/check.sh
```

bootstrap.sh 可重複執行，並以 --locked 確保依賴設定與鎖定檔一致。check.sh 會建置文件、檢查 Rust 格式、執行 Clippy 與測試。

Python 版本檔保留 3.14，讓 pyenv 正常選擇本機版本。腳本只在 uv 指令中加上 3.14+gil，避免既有的 free-threaded 安裝被自動選中。

## 新版 Codex Cloud

先將此專案提交並推送至 GitHub。建立 Cloud environment 時選擇該儲存庫，讓 Codex 使用以下設定：

- Install script：在儲存庫根目錄執行 bash scripts/bootstrap.sh。
- Start skill：任務開始時同步依賴並執行檢查；需要預覽時啟動 Zensical。
- 網路：選擇 Package managers，以便取得 Python、Rust 和套件依賴；額外允許 astral.sh、releases.astral.sh 和 sh.rustup.rs，供缺少 uv 或 rustup 時下載安裝腳本與工具。

可貼入環境設定對話的指示：

```text
請在此儲存庫根目錄執行 bash scripts/bootstrap.sh，
並以 bash scripts/check.sh 驗證 Python 3.14、Zensical 和 Rust 範例。
Start skill 請記錄：
1. 任務開始時執行 uv sync --locked --python '3.14+gil' 與 cargo fetch --locked。
2. 完成變更後執行 bash scripts/check.sh。
3. 需要預覽時執行 uv run --locked --python '3.14+gil' zensical serve --dev-addr 0.0.0.0:8000。
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
uv add --dev 'zensical==<新版本>' --python '3.14+gil'
bash scripts/check.sh
```

提交 pyproject.toml 與 uv.lock 的變更。Rust 依賴或工具鏈更新後，同步檢查 Cargo.lock 和 rust-toolchain.toml。
