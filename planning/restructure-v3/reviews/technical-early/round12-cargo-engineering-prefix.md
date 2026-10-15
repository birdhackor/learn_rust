# 第 12.1 節獨立技術初判

封存：2026-10-15T08:50:00.372Z。必要問題0；可選問題1。

全文、10個完整snippets、獨立workspace根／兩個member／path dependency的全部source、integration test、fixture／expected及答案已靜態核對。21檔SHA-256見JSON。未先讀作者驗證、peer、reader、歷史問題或尚未交接後章。

TC12P-O1（可選，命令範圍／措辭精確度）：正文:150說all-targets包含examples，且「本頁每份程式都已完整列出」；但:156的Clippy已會編譯feature_answer.rs，完整來源要到:188的答案才列出。該檔只用已先教的cfg、單筆／兩筆函式、array與assert，且同頁完整提供，沒有必要新概念或未教allow，所以不是Rust錯誤或必要缺口。建議把句子改成明說後面的完整答案example也會參與檢查，或把整包檢查移至答案source後，方便讀者對照實際載入範圍。

其餘核對通過：共同欄位由member明選、dependency path起點、rust-version/publish、dev/release/profile/overflow、feature列表與default flags、cfg排除與cfg! bool、compile target/host、integrationtest公開入口均在使用前交代並符合actual設定。兩份library只有已教計算，沒有載外層舊package或舊第一章library。根三表節錄已對actual根Cargo.toml值比對一致。

正常run的160／280、no-default移除批次、integration2／1／2、答案zero-first120及E0425負例均與source相符。所有11個Cargo命令與1個rustc負例只作靜態核對，沒有執行。有效Cargo config搜尋路徑無額外設定，scripts/env.sh不設定target-dir，故預設小workspace target/release說法成立。

這是技術整頁重讀，不是盲首讀或browser／讀者實驗。原稿與任何source/config均未修改。
