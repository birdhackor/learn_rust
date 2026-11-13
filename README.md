# Rust 教學

以正體中文撰寫的 Rust 教學專案。文件由 Zensical 建置，程式範例由 Cargo workspace 管理。

[線上閱讀](https://birdhackor.github.io/learn_rust/)｜[課程結構與來源](curriculum/README.md)｜[維護指南](CONTRIBUTING.md)｜[歷史紀錄](ARCHIVES.md)

讀者起點是基本 Python。全書共18章132小節，教學與範例依已教概念排列，進階篇也是完整課程的一部分。最後以重複檔案搜尋器、網址檢查器兩個引導型專案練習，各有可編譯起點、分步驗收、提示與可實際使用的完整參考版。

## 取得教材

只閱讀時使用線上教材。要取得目前範例，可在 GitHub 按 **Code → Download ZIP**；已安裝 Git 的讀者也可只取最新版：

```bash
git clone --depth 1 --no-tags --branch main https://github.com/birdhackor/learn_rust.git
cd learn_rust
```

這個下載保留目前程式，省略舊提交與tag。要查歷史時另用 `git fetch --unshallow --tags`；[歷史審閱封存](ARCHIVES.md)也可獨立按需下載，一般學習與建置不需要它。

## 預覽與驗證

只建置或預覽教材，不需要安裝 Rust 工具鏈：

```bash
bash scripts/bootstrap.sh site
bash scripts/build-site.sh
source scripts/env.sh
uv run --locked --no-default-groups --group site --python '3.14+gil' zensical serve
```

本機預覽網址為 http://127.0.0.1:8000/，網站輸出在 `outputs/site/`。

要修改與驗證 Rust 範例，準備完整環境：

```bash
bash scripts/bootstrap.sh
bash scripts/check.sh
```

`check.sh`驗證發布工具、嚴格建置網站、核對章節／程式引用／舊連結，並驗證全部 Rust／C／故意失敗範例。完整成功結果只在執行輸入與工具環境相同時重用；快取缺少、損壞或來源改動就重新實跑。強制重跑使用 `bash scripts/check.sh --force`。固定nightly的 Miri 另依[維護指南](CONTRIBUTING.md)執行。

工具路徑與快取由 `scripts/env.sh`統一設定；每個新shell直接使用 uv、cargo 或 rustup 前先載入它。首次準備環境需要網路，依賴固定在鎖定檔。

## 工具版本

| 工具 | 專案設定 |
| --- | --- |
| Python | 3.14，一般GIL版本；`.python-version`保留pyenv相容格式 |
| uv | 最低0.13.0；安裝腳本與CI使用0.13.0 |
| Zensical | 0.0.70；`site`群組，版本固定在 `uv.lock` |
| Rust | 1.100.0；版本與元件固定在 `rust-toolchain.toml` |
| Rust edition | 2024 |

## 專案結構

```text
docs/                    教學Markdown、圖解與舊網址入口
examples/                可編譯與測試的Rust範例及必要實測結果
curriculum/              目前章節、先備、來源與舊網址對照
assets/archives/         固定歷史封存的下載與逐檔校驗清單
scripts/bootstrap.sh     all／site／runtime環境準備
scripts/build-site.sh    嚴格建置、連結與來源檢查、build-info
scripts/check.sh         發布工具、網站與Rust／C整合驗證
scripts/check-rust.sh    實跑全部Rust／C／fixture檢查
scripts/check-execution.py 成功結果的保守重用與強制重跑
scripts/archive-assets.py 按需下載、離線驗證歷史審閱
outputs/                 被Git忽略的網站、快取、驗證log及封存
work/                    被Git忽略的暫存、審閱明細與截圖
.github/workflows/       建置、程式驗證及Pages部署
```

完整環境準備後，可執行第一個 Rust 範例：

```bash
source scripts/env.sh
cargo run --manifest-path examples/intro-workspace/Cargo.toml -p intro-hello --locked
```

預期輸出為 `Hello, Rust!`。教材直接引用實際程式檔案。

## Codex Cloud 與 CI

Cloud 的 Install script 使用 `bash scripts/bootstrap.sh`；只維護文字／網站的環境可使用 `bash scripts/bootstrap.sh site`。設定與寫作方式見[維護指南](CONTRIBUTING.md)。

GitHub Actions分開建置網站與驗證程式，兩者通過後才從main發布至[GitHub Pages](https://birdhackor.github.io/learn_rust/)。PR提供檢查與預覽，不部署正式站。網站與驗證產物使用有期限的artifact，歷史審閱不作發布輸入。發布後可讀 `build-info.json`核對版本與建置狀態。
