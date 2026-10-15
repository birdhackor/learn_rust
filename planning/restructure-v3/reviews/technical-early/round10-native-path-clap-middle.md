# 第 10.5–10.6 節獨立技術初判

封存：2026-10-15T08:43:38.975Z。必要問題 0；可選問題 0。

從實際1–9、10.1–10.4接續，不用另讀11章補先備。逐頁核對19個snippet、9份owned Rust完整來源、兩份manifest、三份實際input bytes及missing不存在條件；25個repo source hashes和4個cached clap source hashes見JSON。未先讀作者驗證、peer、reader或歷史問題。

- Path／PathBuf、OsString／OsStr、display與to_str／into_string的用途及ownership準確；原生路徑不必轉UTF8，路徑內容和I/O、cwd也分清。args_os原生Item、完整序列含programname、手寫parser與兩檔名答案均對source。
- stdout資料、stderr診斷與ExitCode、Write／flush／writeln、Cargo --與Bash分流及狀態觀察在首用前完整。主要CLI參數失敗2、檔案/輸出失敗1、成功0，成功只寫原筆記9bytes；診斷寫入若再失敗回1，沒有遞迴報錯。
- clap dependency的derive feature沿已讀10.1。Settings attributes、兩個必要positional與bool long、完整try_parse_from輸入與Into/Clone條件均先教。help/version也是Err但屬stdout正常顯示0，用法錯誤stderr2；print只輸出且可能I/O失敗，main正常返回ExitCode。
- 讀取cached clap4.6.7 source實際核對Parser bound、Error的stream/exit_code/print及version名稱。argv0可決定usage中的intro-clap-stage，version仍用command name顯示query-settings 0.1.0。未借作者validation結論。
- path-cli無lib：早期examples只編譯自身，arguments test不載CLI main。clap-stage正常lib僅Settings與四項tests；main與flag_solution都會編譯它，正文已完整inline，沒有JSON或搜尋helper提前載入。三份argument片段副本bytes相同。

本波覆蓋path-cli五個unique run target與兩個selected test、clap-stage兩個unique run target與一個lib test；共8項unit tests。主要CLI的正例、help/version與失敗狀態是靜態source契約核對，沒有新增Cargo/Rust或CLI執行、沒有產生captured檔案。沒有browser／首讀實驗。

版本補記：封存前AGENTS.md新增獨立lesson-workspace／nested workspace驗證指引，已讀現行全文；舊／新hash在JSON保留。其他本波來源與交接頁hash未變，沒有用後章內容補本波。
