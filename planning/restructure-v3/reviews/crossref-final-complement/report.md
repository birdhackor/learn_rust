# 最終語意引用補充掃描

全文實讀33頁、6073行：第1章排除1.7後7頁、第6章6頁、第10章9頁、第15章6頁、第16章5頁。記錄524個正文引用位置、648個mentions（節號229、章範圍11、相對指引408）。33頁前後hash無變動。

## 確定1項

**XCF-C01 — `docs/chapters/15-06-lesson.md:74`**

> 前五項組態操作可以依序串接；前節已學過的mutable設定與method呼叫在這裡連續使用。build.rs如下：

「前節」實際是15.5。15.5:5明說「本節先做一個獨立的 C 程式」，15.5:11還說「這裡不使用 Rust 的 `let`／`mut`」。真正先前用過的Rust設定操作見10.8:77：「current_dir／arg 修改 Command 的設定，回傳 `&mut Command` 供接續操作；本節直接分行呼叫」。最小改法：**前節→先前**，其餘不動。

## 可選1項

**XCF-O01 — `docs/chapters/15-06-lesson.md:453`**

> 第一行仍借C配置；to_owned沿第10.4節建立獨立Rust String。

10.4:73實際是「這份 byte slice 的 `to_owned()` 建立 `Vec<u8>`」。8.5:123、127直接教ToOwned的`&str`→String，表中寫「把文字內容放進新String」。最小精準化：此處**第10.4節→第8.5節**。ToOwned一般重用可理解，8.5也已在前文，故列可選而非技術錯誤。後半11.3的Drop引用可保留。

其餘已列引用未發現確定的重排語意錯指。保留合法預告；沒有把所有未來引用當作前提錯誤，也沒有按舊mapping換號。

## 證據與界線

- `references.md`／`references.json`：逐頁、逐行原文、目標及核對狀態。
- `actual-target-evidence.json`：必要新版目標的實讀引文、行號與hash；指定33頁保留全文，其他目標按需節錄。既有`target-evidence-part1/2.json`保留初始摘錄。
- `findings.json`：兩項的完整來源／真正目標exact quotes與最小改法。
- `coverage-final.json`：逐頁全文範圍、counts、起末hash與限制；`excluded-candidates.json`保留數值版本等排除理由。

未改repo/source/nav/ledger，未跑Cargo，未spawn，未讀其他audit findings作提示。此輪只查引用語意，沒有擴成全書技術或網站版面審查。
