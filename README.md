# Rust 教學

以正體中文撰寫的 Rust 教學專案。文件由 Zensical 建置，程式範例由 Cargo workspace 管理。

[線上閱讀](https://birdhackor.github.io/learn_rust/)｜[課程規劃](planning/restructure-v3/outline.md)｜[實作與發布進度](planning/restructure-v3/status.md)

讀者起點是基本 Python。全書共 18 章、131 小節，教學與範例依已教概念排列，進階篇也是完整課程的一部分。最後以重複檔案搜尋器、網址檢查器兩個引導型專案練習，各有可編譯起點、分步驗收、提示與可實際使用的完整參考版。

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
scripts/check.sh        文件建置、結構、格式、Clippy 與測試
scripts/check-tutorial-structure.py 新章節、舊入口、來源與審閱指紋
scripts/check-fixtures.py 故意失敗範例的編譯／panic 契約
scripts/check-c-examples.py C11 正常程式與指定編譯失敗
scripts/check-miri.sh    固定 nightly 的純 Rust 路徑與 Miri 負例
planning/               課程、來源研究、作者先備與審閱紀錄
.github/workflows/      GitHub Actions 建置檢查
```

執行第一個 Rust 範例：

```bash
source scripts/env.sh
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-hello --locked
```

預期輸出為 Hello, Rust!。教學文件直接引用 examples/intro-workspace/hello/src/main.rs。

## Codex Cloud 與 CI

Cloud 的 Install script 可使用 bash scripts/bootstrap.sh。Start skill 的設定、Legacy 環境和寫作方式見 [貢獻指南](CONTRIBUTING.md)。

GitHub Actions 會檢查文件與 Rust 範例，並提供 documentation-site 建置產物。main 通過檢查後，自動發布至 [GitHub Pages](https://birdhackor.github.io/learn_rust/)；PR 只提供檢查與預覽。
