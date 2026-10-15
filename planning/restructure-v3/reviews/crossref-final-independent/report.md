# 新版 canonical 章節引用：獨立只讀收尾

本次掃描章2–5、7–9、11–14、17、18A（18.1–18.6）。作者自己的1／6／10／15／16不作受審來源；它們只在被引用時作目標核對。18B沒有實稿，依指示不驗其未來目標。未閱讀他人的finding來尋題，未改repo任何檔，未執行Cargo／rustc／Miri／Loom或測試，未spawn。

找到1項確定的相對範圍錯指，root已採納並修改；只讀重看確認現在已明指7.1–7.2。本次未找到其他確定錯章，也未另列可選文字潤飾。這是引用語意審閱，不是全書程式、課序或工具結果的重新獨立驗收。

## XREF-I-001：7.4的「前兩節」實際是本章7.1–7.2（已由root修正）

- 觀察位置：`docs/chapters/07-04-lesson.md:114`。
- 原句：完整 `src/source.rs` 收入前兩節的函式。測試現在用第 6.2 節的 `&str` 內容比較，包含等長時選第一段，避免只比長度而漏掉來源選擇錯誤：
- 問題：在7.4中「前兩節」通常指7.2–7.3；此檔實際收7.1–7.2的來源函式與7.2練習。7.3轉入TextView模型。
- 真目標原句：`07-01-lesson.md:86`：「完整 helper 只有 same 與 longer」；`07-02-lesson.md:64`：「以下 copy_text 借讀來源，用第 3.8 節的 `String::new()` 與 `push_str` 建立另一份 owned 文字，再把它 move 給呼叫者」；`07-02-lesson.md:166`：「寫 `first_nonempty`：第一段非空就回傳第一段；第一段為空才回傳第二段」。`07-03-lesson.md:5`則是「lifetime parameter 也能寫進 struct」。
- 實際source：`examples/intro-workspace/lifetimes/complete/src/source.rs:1/5/13/17/23/27/35`依次是same、longer、first_text、copy_text、same_placeholder、fallback、first_nonempty。已親讀完整檔，沒有只靠檔名判斷。
- 最小改法：將「收入前兩節的函式」改為「收入第 7.1–7.2 節的函式」。目前root已作此修改；原觀察hash及修後hash保存在findings.json，不把修後原句冒充最初無問題。

## 覆蓋與方法

逐頁親讀全部候選引用句，對照實際新版目標的能力定義、必要source與本頁鄰接段落。數字存在不等於語意成立；特別分開外部Book節號、版本字串、自己H1編號、未寫18B及真正教材引用。正確項保留逐頁count與行號，不複製整本正文。

這次也重讀12／13／17重要能力段落、13.2／13.3目前全文，以及有疑義的source。先前由本審閱者親讀的前綴正文/source，只在目前SHA完全相同時重用實讀紀錄；本次讀過的引用句仍逐一核對。保存在reused-personal-read-evidence.json的先前讀檔不表示本次把每個檔案全文再讀一次。部分新目標採非fence主文核對，target-evidence.json已註明，不冒稱Rust全closure重新審查。

初始盤點：91頁、1389個詞法命中。最終快照：91頁、16,795行、1390個詞法命中，分布於1191行；另補讀46行章範圍／『前面的』『前面已』『最後一節』等相對說法。詞法命中含非引用項，不把它們當成同樣數量的獨立語意主張。

| 章 | 頁數 | 詞法命中 | 命中行 | 補充行 |
| --- | ---: | ---: | ---: | ---: |
| 2 | 8 | 57 | 54 | 3 |
| 3 | 8 | 61 | 60 | 0 |
| 4 | 7 | 76 | 67 | 2 |
| 5 | 5 | 55 | 49 | 1 |
| 7 | 4 | 62 | 55 | 1 |
| 8 | 6 | 104 | 88 | 5 |
| 9 | 7 | 113 | 100 | 2 |
| 11 | 11 | 135 | 125 | 3 |
| 12 | 4 | 97 | 82 | 3 |
| 13 | 8 | 196 | 150 | 8 |
| 14 | 9 | 108 | 100 | 4 |
| 17 | 8 | 191 | 157 | 10 |
| 18 | 6 | 135 | 104 | 4 |

最終分類：

- `canonical_chapter`：48。
- `canonical_section`：677。
- `external_book_section`：1。
- `local_forward_reference`：310。
- `planned_18B_out_of_scope`：2。
- `relative_reference`：248。
- `self_heading`：91。
- `version_not_chapter`：13。

## 重排區的核對結論

- 12.1的workspace繼承／features／cfg／profile與12.2的build script／host區分，能支持後續manifest與proc-macro回指；12.3只掃描，12.4才處理bytes/hash/真正內容群與metrics。
- 13.1 Send／Sync、13.2 spawn／join／panic、13.3 HRTB、13.4 scoped借用、13.5 channel、13.6 Arc／Mutex、13.7 RwLock、13.8 atomic有各自實際正文。未依舊章號機械替換。
- 17.6的「各增加一次」對應13.8 lost-update段，不是後面的五次計數範例。13.8:57確實寫「原需求是兩份工作各增加一次，初值0應成為2」。其後用同thread手工安排交錯的角色也已寫明，沒有把兩段數值不同當成錯引用。
- 14.8的std Instant確實在14.5:11介紹，Tokio虛擬時鐘在後段另分。14.9的IntoFuture／Option::take是當節新教，不誤稱早期已教。
- 18.5的CARGO_BIN_EXE與--test可回10.8，並非只能回14.2；18.6的Source<'_>回傳模型省略能回7.3:25。12.4的folder example規則在2.6確有實際段落。
- 11.2回3.6的shared reference不得移出non-Copy值，以及11.4回11.3的Drop型別不得直接移出欄位，均有實際負例與說明，未誤報成應改指其他節。

## 產物與限制

- `findings.json`：確定問題的exact path／line／原句、真目標原句、最小改法、初始及修後SHA。
- `coverage.json`、`chapter-coverage.json`：91頁逐頁行號、count、分類及章節判斷。
- `inventory.json`：最初盤點；`final-inventory.json`：完成時目前稿，保留每個詞法命中及補充句。
- `target-evidence.json`：重排及容易誤判項的實際目標範圍／hash／採信理由。
- `reused-personal-read-evidence.json`：本審閱者先前實讀且目前SHA相同的來源，與本次重讀區別。
- `snapshot-drift.json`：審閱期間root／其他作者修改的頁面，已重看目前引用，不把自己不存在的repo改動列為修正。

這是靜態引用語意核對；沒有重新執行程式、驗證外部網站當前內容、重新量測效能或重跑工具。沒有把所有正常引用逐字抄進報告，也沒有宣稱整個91頁的每一份Rust完整closure都在此輪重新全文審過。18B未有實稿，所以18.1的18.11／18.12前指只記錄為範圍外待核，沒有當成已通過。後續稿件若再改動，需以新hash重新判斷受影響的引用。

任務完成，STOP，不續寫任何章節。
