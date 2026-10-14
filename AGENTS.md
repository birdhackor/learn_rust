# 專案協作規則

- 請用英文思考／規劃，但用正體中文與使用者溝通。
- 教學內容與維護文件使用正體中文；Rust API、指令與識別字保留原文。
- 撰寫或審閱 Rust 教材時，使用 [clear-tutorial](.agents/skills/clear-tutorial/SKILL.md)；中英文術語依該 skill 的 [Rust 術語與語言指引](.agents/skills/clear-tutorial/references/rust-terminology.md)。更新 skill 與專案指引不代表教材已改寫或通過審閱。
- 文件來源在 docs/，站點設定在 zensical.toml。
- 可執行 Rust 範例放在 examples/，並加入根目錄 Cargo workspace。
- 文件用 pymdownx.snippets 引用範例原始碼，避免維護兩份相同程式。
- 片段引用以專案根目錄為基準；缺少來源檔案必須讓建置失敗。
- .python-version 保持與 pyenv 相容的 3.14；uv 指令可用 --python '3.14+gil' 選擇一般版本。
- Python 依賴透過 uv 管理，修改依賴後同步更新 uv.lock。
- 工具路徑與 Python／Rust 快取由 scripts/env.sh 統一設定。每個新 shell 直接執行 uv、cargo 或 rustup 前先 source scripts/env.sh；bootstrap.sh 與 check.sh 會自動載入。不要假定安裝階段的 export 會沿用到新任務。
- Rust 工具鏈由 rust-toolchain.toml 固定，提交 Cargo.lock。
- 新章節加入 zensical.toml 的 nav，清楚描述學習目標、範例和練習。

## 開發指令

首次準備環境：

```bash
bash scripts/bootstrap.sh
```

文件與程式碼變更完成後執行：

```bash
bash scripts/check.sh
```

本機預覽：

```bash
source scripts/env.sh
uv run --locked --python '3.14+gil' zensical serve
```

Codex Cloud 設定與寫作流程見 CONTRIBUTING.md。
