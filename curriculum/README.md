# 課程結構與來源

目前教材共18章131節。`sections.json`保存章節順序、先備、小節路徑與實際程式來源；`migration-map.json`及`legacy-anchors.json`保留109個舊入口與890個段落anchor。這些檔案供目前教材的結構檢查使用，不表示已重新完成教學審閱。

| 章 | 主題 | 小節數 |
| --- | --- | --- |
| 1 | [Rust 起步、基本語法與計算機基礎](../docs/chapters/01-01-lesson.md) | 8 |
| 2 | [流程控制：條件、迴圈與可測試程式](../docs/chapters/02-01-lesson.md) | 8 |
| 3 | [ownership、reference與borrowing](../docs/chapters/03-01-lesson.md) | 8 |
| 4 | [enum與pattern matching](../docs/chapters/04-01-lesson.md) | 7 |
| 5 | [泛型、Option／Result與可變長度資料](../docs/chapters/05-01-lesson.md) | 5 |
| 6 | [trait、標準能力與HashMap](../docs/chapters/06-01-lesson.md) | 6 |
| 7 | [lifetime annotation：回傳借用與資料模型](../docs/chapters/07-01-lesson.md) | 4 |
| 8 | [錯誤、格式化與轉換](../docs/chapters/08-01-lesson.md) | 6 |
| 9 | [closure與Iterator](../docs/chapters/09-01-lesson.md) | 7 |
| 10 | [Cargo生態系、I/O與第一個CLI](../docs/chapters/10-01-lesson.md) | 9 |
| 11 | [smart pointer、資源與進階型別](../docs/chapters/11-01-lesson.md) | 11 |
| 12 | [Cargo工程設定與檔案處理](../docs/chapters/12-01-lesson.md) | 4 |
| 13 | [thread、同步與atomic](../docs/chapters/13-01-lesson.md) | 8 |
| 14 | [async、HTTP與受控測試](../docs/chapters/14-01-lesson.md) | 9 |
| 15 | [進階lifetime、unsafe與FFI](../docs/chapters/15-01-lesson.md) | 6 |
| 16 | [Pin、Future與executor內部](../docs/chapters/16-01-lesson.md) | 5 |
| 17 | [macro、API設計、驗證與效能](../docs/chapters/17-01-lesson.md) | 8 |
| 18 | [兩個引導式完整工具](../docs/chapters/18-01-lesson.md) | 12 |

## 作者來源索引

下列檔案保留作者當時實際查閱的來源、採用方式與限制。`original`使用重排前的章號，`restructure-v3`記錄重排時的補充；它們是有日期的來源紀錄，不是讀者必備前文，也不宣稱這次又讀過來源或重新驗收教材。紀錄中的舊審閱／工作區路徑依[封存索引](../ARCHIVES.md)追溯，原始文字保留。

- [第 1 章作者來源閱讀紀錄](sources/original/ch01.md)
- [第 2 章作者教材研究紀錄](sources/original/ch02.md)
- [第 3 章教材研究紀錄](sources/original/ch03.md)
- [第 4 章：作者動筆前的教材閱讀紀錄](sources/original/ch04.md)
- [第 5 章作者教材研究紀錄](sources/original/ch05.md)
- [第 6 章教材研究紀錄](sources/original/ch06.md)
- [第 7 章作者的來源研究](sources/original/ch07.md)
- [第 8 章：作者動筆前的教材閱讀紀錄](sources/original/ch08.md)
- [第 9.2–9.4 節作者追加來源與先備](sources/original/ch09-author02.md)
- [第 9 章來源研究與最終實戰 A 的前置](sources/original/ch09.md)
- [第 10 章作者研究紀錄：thread、同步與 atomic](sources/original/ch10.md)
- [11.4 作者實讀來源：時間、timeout 與受控驗證](sources/original/ch11-author01.md)
- [11.7 作者來源：把單一 HTTP 工作接到 async](sources/original/ch11-author07.md)
- [第 11 章來源閱讀紀錄](sources/original/ch11.md)
- [第 12 章：作者動筆前的教材閱讀紀錄](sources/original/ch12.md)
- [第 13 章：作者動筆前的教材閱讀紀錄](sources/original/ch13.md)
- [14.4／14.5 作者來源與取捨](sources/original/ch14-api-author01.md)
- [第 14 章作者追加研究：macro、公開 API 與驗證工具](sources/original/ch14-author04.md)
- [第 14.1–14.3 作者 fresh 來源閱讀](sources/original/ch14-macros-author04.md)
- [14.7／14.8 作者來源研究：公平量測與改善證據](sources/original/ch14-performance-author01.md)
- [14.8 作者來源：用配置profile核對可移除的工作](sources/original/ch14-profiling-author01.md)
- [14.6 Loom／Miri 作者實讀來源](sources/original/ch14-tools-author04.md)
- [第 14 章來源研究：macro、API、驗證與效能](sources/original/ch14.md)
- [最終實戰 A：引導寫作與驗收來源研究](sources/original/ch15-a.md)
- [15.7 B1 作者來源：一份 URL 與一份 owned 結果](sources/original/ch15-b-author07.md)
- [第 15 章 B 專案：來源與驗收研究](sources/original/ch15-b.md)
- [15.8 B2 作者來源：先保留輸入，再依序檢查](sources/original/ch15-b2-author07.md)
- [15.9 B3 作者來源：有限 task 與完整輸入對應](sources/original/ch15-b3-author07.md)
- [15.10 B4 作者來源：觀察停止、保存結果、等待收尾](sources/original/ch15-b4-author07.md)
- [15.11 B 完整工具：作者來源與取捨](sources/original/ch15-final-b-author07.md)
- [第 1 章 v3 作者來源實讀紀錄](sources/restructure-v3/sources-ch01.md)
- [新第 2 章作者來源實讀紀錄](sources/restructure-v3/sources-ch02.md)
- [第 3 章來源實讀與取捨](sources/restructure-v3/sources-ch03.md)
- [第4章來源實讀與取捨](sources/restructure-v3/sources-ch04.md)
- [第5章來源實讀與取捨](sources/restructure-v3/sources-ch05.md)
- [第 6 章 v3 作者來源與前綴實讀](sources/restructure-v3/sources-ch06.md)
- [新第 7 章來源、實讀範圍與採用界線](sources/restructure-v3/sources-ch07.md)
- [第8章來源實讀與取捨](sources/restructure-v3/sources-ch08.md)
- [第9章外部來源與實讀證據](sources/restructure-v3/sources-ch09.md)
- [第10章v3來源實讀 ledger](sources/restructure-v3/sources-ch10.md)
- [新第11章來源：11.1–11.2實作交接](sources/restructure-v3/sources-ch11.md)
- [第12章外部來源與實讀證據](sources/restructure-v3/sources-ch12.md)
- [第13章來源實讀與取捨](sources/restructure-v3/sources-ch13.md)
- [第14章來源與實讀範圍](sources/restructure-v3/sources-ch14.md)
- [第15章v3來源實讀 ledger](sources/restructure-v3/sources-ch15.md)
- [第16章v3來源實讀](sources/restructure-v3/sources-ch16.md)
- [第17章來源實讀與取捨](sources/restructure-v3/sources-ch17.md)
- [第18章A外部來源實讀](sources/restructure-v3/sources-ch18a.md)
- [第18章B實讀來源與寫作取捨](sources/restructure-v3/sources-ch18b.md)
- [1.7 外部來源實讀與採用紀錄](sources/restructure-v3/sources-foundations.md)

新增或改寫教材時，更新目前結構與必要來源索引；詳細審閱、指紋、截圖及完整log依[貢獻指南](../CONTRIBUTING.md)保存。歷史審閱不作網站建置輸入。
