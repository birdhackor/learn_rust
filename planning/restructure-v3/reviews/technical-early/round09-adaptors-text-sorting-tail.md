# 第 9.5–9.7 節獨立技術初判

封存：2026-10-15T08:35:16.264Z。必要問題 0；可選問題 0。

從實讀 1–8、9.1–9.4 接續，逐頁核對 31 個 snippet、16 個 run example、4 組負例與 expected。38 檔 SHA-256、9 份 helper 片段的 bytes 比較與完整 closure 見 JSON。未先讀作者驗證、peer、reader 或歷史問題。

- map／filter／collect／fold 的 Item 與 callback 契約一致；filter 的 &&i32、owned String 的 &String、下一步 map 的 &i32 都有先教。FromIterator 的目標選擇、Vec Debug／PartialEq、兩種 turbofish 路徑都在實用前說明。fold 的具名局部 Clippy allow 也在首次 source 前解釋。
- Result collect 的首錯短路、空來源、無部分 Vec、無 From 錯誤轉換正確。實際 source 第二次 parse 即失敗，visits=2，借用來源仍保有三筆；答案 ? 的相同型別 From 回指已讀8.6。
- Unicode scalar、Debug escape、LF／CRLF／孤立 CR、尾行與空行的敘述和 source 一致。Entry 的 ownership、FnOnce callback、借用與 lazy value／eager key 差別完整。三種 HashMap 走訪沒有冒稱固定順序或 mutable key。
- Ordering／PartialOrd／Ord／NaN、三種 stable sort 與 sort_unstable 界線準確。報表以次數降序再名稱升序解決 HashMap 任意來源順序；完整答案僅反轉次數比較，名稱與字串 ownership 保留正確。

intro-iterators 仍為無 dependencies、無 src/lib.rs／build.rs 的 no-lib package；16 個 target 皆只有正文完整提供的單檔 source，未載 sibling 或未讀 library。沒有新增 Cargo/Rust 執行。pipeline SVG 僅讀 XML/source，不稱視覺或讀者驗證。
