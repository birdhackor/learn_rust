# 第 12.2 節獨立技術初判

封存：2026-10-15T09:02:29.638Z。所讀版本必要問題0；可選問題0。

首次actual閱讀hash為846d2893b7c8c692a00efa7792cf4256cde6732e35f118d104585ed72bdfba7a。root交接已通知曾窄改command時序；本次只審新稿，不把原問題／修正算自己獨立發現。未讀作者validation、peer、reader或歷史問題。

全文、8個完整snippets、package全部7份Rust（build.rs、library、main、兩examples、integrationtest、fixture）與manifest／根共同設定／lock已核對。20檔SHA-256及逐command exact closure見JSON。所有API／設定角色都在必要首用前交代。

四階段準確：host編譯並執行script，先驗證有限LESSON_MODE，再以cargo::協定交付值，主要crate env!嵌入static文字，成品只顯示。CARGO_CFG_TARGET_OS與script cfg!的host條件分清；缺值、非Unicode、錯mode、cargo::error與option_env!政策均對source。

normal lib自動建置、script重跑追蹤、編譯／runtime環境差別、OUT_DIR／build-dependencies／linker導覽界線無誤。--test embedded此時不提前選examples；CARGO_PKG_NAME與兩份example都完整教完後才all-targets。Linux/Windows兩tests、其他OS一test的cfg條件一致。

沒有新增Cargo/Rust或成品執行。8個Cargo命令、1個直接成品run與1個rustc負例只作靜態核對。build-phases.svg只讀XML/source，不稱browser或讀者實驗。
