# Rust 教學

以正體中文撰寫的 Rust 教學專案。文件由 Zensical 建置，程式範例由 Cargo workspace 管理。

## 開始開發

在專案根目錄執行：

```bash
bash scripts/bootstrap.sh
bash scripts/check.sh
source scripts/env.sh
uv run --locked --python '3.14+gil' zensical serve
```

本機預覽網址為 http://127.0.0.1:8000/。網站輸出在 site/。

bootstrap.sh 會在 uv 缺少或版本不足、rustup 缺少時安裝工具，明確安裝一般 GIL 版本的 Python 3.14，依鎖定檔同步 Python 依賴，並準備 Rust 工具鏈與套件快取。首次執行需要網路。工具路徑與快取由 scripts/env.sh 統一設定；每個新 shell 直接使用 uv 或 cargo 前先載入它。

## 工具版本

| 工具 | 專案設定 |
| --- | --- |
| Python | 3.14，一般 GIL 版本；.python-version 保留 pyenv 相容格式 |
| uv | 最低 0.13.0；安裝腳本與 CI 使用 0.13.0 |
| Zensical | 0.0.70；所有 Python 依賴固定在 uv.lock |
| Rust | 1.99.0；版本與元件固定在 rust-toolchain.toml |
| Rust edition | 2024 |

## 專案結構

```text
docs/                    教學 Markdown
examples/                可編譯與測試的 Rust 範例
zensical.toml            站點、導覽與 Markdown 設定
pyproject.toml / uv.lock 文件工具依賴
Cargo.toml / Cargo.lock  Rust workspace 與依賴
scripts/bootstrap.sh    本機、Cloud 與 CI 的環境準備
scripts/env.sh          共用工具路徑與快取設定
scripts/check.sh        文件建置、格式、Clippy 與測試
.github/workflows/      GitHub Actions 建置檢查
```

執行第一個 Rust 範例：

```bash
source scripts/env.sh
cargo run -p hello-world --locked
```

預期輸出為 Hello, Rust!。教學文件直接引用 examples/hello-world/src/main.rs。

## Codex Cloud 與 CI

Cloud 的 Install script 可使用 bash scripts/bootstrap.sh。Start skill 的設定、Legacy 環境和寫作方式見 [貢獻指南](CONTRIBUTING.md)。

GitHub Actions 會檢查文件與 Rust 範例，並提供 documentation-site 建置產物。發布網址確定後，可再設定 site_url 與部署流程。
