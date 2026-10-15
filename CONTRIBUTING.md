# 維護與 Codex Cloud 開發

## 本機與 Cloud 共用流程

```bash
bash scripts/bootstrap.sh
bash scripts/check.sh
```

bootstrap.sh 可重複執行，會檢查 uv 是否至少為 0.13.0，缺少或版本不足時安裝固定版本；再明確安裝一般 GIL 版本的 Python 3.14，並以 --locked 確保依賴設定與鎖定檔一致。check.sh 會建置文件、檢查 Rust 格式、執行 Clippy 與測試，再確認故意失敗的 Rust 範例、C11 正常程式與指定編譯失敗都符合契約。

第14章的 Miri 另用固定 nightly 執行，CI 也包含這項檢查。在已載入 scripts/env.sh 的 shell 中：

```bash
rustup toolchain install nightly-2026-10-01 --profile minimal --component miri --component rust-src
bash scripts/check-miri.sh
```

故意失效的 pointer 案例僅交給 Miri 解譯，不以普通 cargo run／test 執行。

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

環境有可用的瀏覽器時，直接檢查真實頁面；沒有時，另用本機預覽補驗。建置成功與 HTTP 回應不能代替桌面／手機的視覺與閱讀路徑檢查。

## 新增教學章節

1. 先讀 [clear-tutorial](.agents/skills/clear-tutorial/SKILL.md)、術語指引及對應的中英文教材，保存實際來源與借鑑內容。依實際已寫出的前文核對先備；大綱上的節點不等於已教。
2. 每節正文放 `docs/chapters/NN-MM-*.md`，真實程式放 `examples/chNN-*/`。範例、練習答案、測試與設定都只使用此處之前已教的能力；本節的新能力先說明用途與契約，再用於程式。
3. 文件以 snippets 引用實際原始碼，使用 rust 語法標示；引用路徑相對於專案根目錄。後節新增能力時保留早節範例，不讓共享source把未教的derive／API帶回前頁。
4. 平行作者依 `planning/implementation-status.md` 分工，各自保存先備紀錄；由協調者整合 Cargo workspace／lock與導覽。來源研究可先平行，正文與程式等真正的前置稿完成才寫。
5. 完成後執行 bash scripts/check.sh，另實跑有意義的輸入與輸出。凍結版本，再分開保存順序首讀、技術與銜接初判及修後複查；實看桌面與手機的SVG／圖文位置。導覽的公開釋出另行確認。

故意不能編譯的最小範例放在 `fixtures/compile-fail/`；故意panic的範例放在 `fixtures/runtime-fail/`。每個 `.rs` 配同名 `.expected`，填必要錯誤碼或panic文字。它們不是正常Cargo target；檢查器會確認失敗原因符合正文，不因任意錯誤就判通過。

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

## GitHub Pages 發布

首次設定時，至儲存庫 Settings → Pages，將 Build and deployment 的 Source 設為 GitHub Actions。公開站點為 https://birdhackor.github.io/learn_rust/。

推送 main 或手動執行 Check and publish Rust documentation 後，workflow 先建置文件、檢查 Rust 格式、執行 Clippy 與測試；全部通過後才發布 Pages。PR 只檢查與上傳預覽，不部署正式網站。

部署使用 GitHub Actions 自帶的 GITHUB_TOKEN，workflow 僅在部署 job 授予 pages: write 與 id-token: write，不需要額外 repository secret。啟用 Pages 的管理操作須由具有權限的使用者先完成。

未發布章節以 zensical.toml 的 `exclude_docs` 排除；通過審閱後，加入相應 `!chapters/NN-*.md` 規則與 nav。維護者本機可以暫開草稿導覽做畫面檢查，推送main時以實際已驗收的批次為準。公開進度與部署證據記在 `planning/implementation-status.md`。
