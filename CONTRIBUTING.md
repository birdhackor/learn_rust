# 維護、審閱與發布

## 環境準備

在專案根目錄選擇需要的工具：

```bash
bash scripts/bootstrap.sh          # all：網站及Rust／C驗證
bash scripts/bootstrap.sh site     # 網站工具，不安裝Rust
bash scripts/bootstrap.sh runtime  # Python及Rust工具，不安裝網站套件
```

腳本可重複執行。uv缺少或低於0.13.0時安裝固定版本，明確安裝一般GIL版Python 3.14，再以 `--locked`同步依賴。`site`群組只含網站需要的Zensical，`dev`包含此群組供一般開發。`.python-version`保留pyenv相容的 `3.14`；uv以 `3.14+gil`選擇一般版本。

所有shell腳本載入 `scripts/env.sh`，優先使用儲存庫上一層的 `.learn-rust-tools/bin`。工具位置可用 `LEARN_RUST_TOOLS_DIR`指定；已設定的 `CARGO_HOME`、`RUSTUP_HOME`、`UV_CACHE_DIR`與 `UV_PYTHON_INSTALL_DIR`會保留。安裝時的export不沿用到下一個shell或Cloud任務，直接使用工具前先執行：

```bash
source scripts/env.sh
```

## 網站與程式驗證

```bash
bash scripts/build-site.sh
bash scripts/check.sh
```

`build-site.sh`真正執行Zensical嚴格建置與結構檢查，不重用舊HTML。它核對目前章節、先備順序、實際snippet來源、導覽、站內連結、圖檔、重複id、109個舊入口及890個原anchor；成功後才寫入 `outputs/site/build-info.json`。版本資訊包含Git commit、工作樹是否修改、Zensical版本與結構檢查範圍；沒有Git資料的ZIP環境明記版本未知。

`check.sh`另執行發布工具的離線單元測試，並以 `check-execution.py`驗證或實跑 `check-rust.sh`。後者檢查四個workspace的fmt、Clippy、全部targets及文件測試，再核對Rust故意失敗範例與C11正常／指定失敗案例。缺少snippet、原始碼或失敗契約仍會失敗。

只驗程式時：

```bash
source scripts/env.sh
uv run --locked --no-default-groups --python '3.14+gil' python scripts/check-execution.py ensure --kind rust
```

### 成功結果的重用

每一類檢查只有一份完整結果，不建立逐章跳過規則。鍵包含 `examples/`、`scripts/`、`tests/`、`.cargo/`、`.github/`、非Markdown文件資產、根manifest／lockfile／工具鏈，以及實際Python、Rust、Cargo、C編譯器或Miri版本、平台與相關環境設定；Cargo的上層／全域設定及指定的compiler wrapper也綁定內容雜湊。新增執行輸入若放在其他路徑，也要更新 `INPUT_DIRECTORIES`或 `INPUT_FILES`與失效測試。

正文文字、來源筆記及按需封存不影響程式執行鍵。重用前仍核對成功狀態、完整命令、執行時間、鍵與log內容雜湊；缺損、過期或檢查失敗就重跑。執行期間輸入若改變，不保存成功結果。紀錄包含當時commit與工作樹狀態，環境值只影響摘要，不寫入紀錄。

強制實跑全部Rust／C檢查：

```bash
bash scripts/check.sh --force
```

結果與完整log寫在被忽略的 `outputs/checks/`。移除對應的成功紀錄也會使下次重跑。這是相同程式輸入的執行結果，不證明新文字已通過教學審閱。

### Miri

固定nightly另行準備與執行，CI也包含此項：

```bash
source scripts/env.sh
rustup toolchain install nightly-2026-10-01 --profile minimal --component miri --component rust-src
uv run --locked --no-default-groups --python '3.14+gil' python scripts/check-execution.py ensure --kind miri
```

加上 `--force`可忽略成功快取。Miri故意失效的pointer案例僅交給Miri解譯，不以普通 `cargo run`或 `cargo test`執行。

工具鏈與依賴已準備完成後，可以離線驗證：

```bash
UV_OFFLINE=1 CARGO_NET_OFFLINE=true bash scripts/check.sh
```

## 預覽與實頁檢查

```bash
source scripts/env.sh
uv run --locked --no-default-groups --group site --python '3.14+gil' zensical serve --dev-addr 0.0.0.0:8000
```

雲端的localhost指向雲端環境。本機瀏覽器須使用轉送入口，或下載CI的 `documentation-site`產物後啟動HTTP伺服器。建置成功與HTTP回應不能代替桌面／手機的視覺及閱讀路徑檢查；依改動範圍實看，未做的部分記為未驗證。

## 新增或改寫教材

1. 先讀 [clear-tutorial](.agents/skills/clear-tutorial/SKILL.md)、Rust術語指引及實際借鑑的中英文教材。必要先備依已寫出的正文核對，不把大綱上的名稱當成已教。
2. 正文放 `docs/chapters/NN-MM-lesson.md`，可執行程式放 `examples/`。範例、答案、測試及設定只使用此處以前已教的能力；新能力先說明用途與契約。來源依目前章號保存在 `curriculum/sources/`，記實際來源、範圍、採用方式與限制。
3. 用snippets引用實際程式；路徑相對於專案根目錄，缺少來源必須失敗。保留早節階段版本，不讓後節新增能力滲回前文。
4. 更新 `curriculum/sections.json`及 `zensical.toml`的順序、標題、先備、執行命令與實際載入來源。舊網址以遷移對照與anchor清單維護，不建立第二套正文。
5. 依改動範圍進行首讀、技術與銜接審閱、必要修正及修後複查；實際執行與視覺檢查分開記。未進行獨立審閱時標為作者自查。教材驗收與程式檢查完成後才納入正式導覽及發布。

入門使用獨立的 `examples/intro-workspace/`，完整member manifest、library、helper及測試皆只載入當時已教能力。第12章的兩個lesson workspaces也獨立驗證。根workspace的全書檢查是維護者驗收，不要求初學者在早章先編譯後章。

故意不能編譯或panic的最小範例放在對應 `fixtures/compile-fail/`或 `fixtures/runtime-fail/`，每份 `.rs`須配同名非空 `.expected`。負例檢查器確認指定原因，不把任意錯誤算通過。C範例與Miri負例依各自的檢查腳本驗證。

### 審閱資料的保存

詳細逐段紀錄、版本指紋、快照、完整log、截圖及臨時工具放在 `work/`，需要協作時上傳有期限的artifact；不把每輪審閱鏈或外部教材全文提交回主線。保留原始判斷與修後複查，不覆寫成最初就沒有問題。

PR留下範圍、主要結論、必要修正、實際驗證及未驗證限制，並連結需要的artifact。首讀、技術、銜接、程式實跑與實頁觀察各自說明，不以檢查器通過宣稱已理解或取得學習成效。發布不重放歷史審閱，也不依賴審閱者身分、逐行引文或凍結檔案的舊指紋。

來源、授權及支持公開教材主張的必要實測資料須長期保存。既有重排與審閱資料的固定commit及按需下載方式見 [ARCHIVES.md](ARCHIVES.md)；一般建置、測試與學生操作不自動下載封存。

## 更新依賴

```bash
source scripts/env.sh
uv add --group site 'zensical==<新版本>' --python '3.14+gil'
bash scripts/check.sh
```

提交 `pyproject.toml`與 `uv.lock`。Rust依賴或工具鏈更新時同步核對 `Cargo.lock`、`rust-toolchain.toml`與CI的固定版本；執行鍵會因此失效。

## Codex Cloud

新版環境的Install script使用 `bash scripts/bootstrap.sh`；只做網站時可選 `site`模式。Start skill記錄每個新shell先載入 `scripts/env.sh`，開始時同步所需依賴，完成後執行適合改動的驗證。Legacy環境的Setup／Maintenance script也可使用相同bootstrap。

網路選擇Package managers，並允許 `astral.sh`、`releases.astral.sh`與 `sh.rustup.rs`供缺少工具時安裝。只在主動取得歷史封存時才需要 `codeload.github.com`。環境設定修改後重新測試及Republish；儲存庫更新不自動重跑安裝。

詳見[新版Cloud environments](https://learn.chatgpt.com/docs/environments/cloud-environments)與[Legacy文件](https://learn.chatgpt.com/docs/environments/cloud-environment)。

## GitHub Pages發布

首次設定在Settings → Pages將Source設為GitHub Actions。公開站點為 https://birdhackor.github.io/learn_rust/。

推送main或手動執行workflow後，網站job以最小依賴建置並檢查目前教材；runtime job驗證或重新實跑完整Rust／C／Miri結果。兩者通過才部署同一次workflow的Pages artifact。PR只提供檢查與預覽。手動執行的 `force_checks`可強制重跑程式檢查。

CI使用淺層checkout，不要求舊審閱或完整Git歷史。Cargo快取只重用下載與建置產物；完整成功結果以精確執行鍵另存，不使用模糊fallback。預覽及驗證log保留7天，不作長期重現資料的唯一保存處。

部署使用Actions自帶的 `GITHUB_TOKEN`，只有deploy job取得 `pages: write`與 `id-token: write`。發布後核對線上的 `build-info.json`、新頁面、舊入口與必要操作；本地通過不代表CI或公開部署已完成。
