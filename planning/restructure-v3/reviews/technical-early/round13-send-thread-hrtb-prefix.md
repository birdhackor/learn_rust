# 第 13.1–13.3 節獨立技術初判

封存：2026-10-15T09:11:04.523Z。必要問題0；可選問題0。

三節全文、61snippets、15selectedrun及1selectedtest（1函式／3payload cases）、12組完整compile-fail／expected已靜態核對。70檔SHA-256、逐target閉包和8份helper bytes比較見JSON。未讀作者驗證、peer、reader或歷史問題；未以13.4+或12.3+補先備。

Send／Sync與static、Fn呼叫能力各自角色準確；Cell/RefCell的Send依T而非Sync、Rc計數非跨thread契約、move不會修Send，都對actualsource。最初helper沒有實際建立thread，文字與圖未把contract check冒充並行實測。

spawn的F/T兩組Send/static、JoinHandle消耗等待、unwind／abort、外thread::Result與內業務Result、panic payload三種處理與stderr診斷界線都先教。detach不等待不取消，答案先join兩份再判結果。u32及型別註記已在原1.4中教，closure回傳箭頭已由9.1教；13.2新增literal suffix也在test首用前說明。

HRTB準確區分外層'a和bound內for、固定T和隨輸入'a的輸出、capture借用和callback輸入通用性、FnOnce/FnMut/Fn呼叫能力。外部String view可保留，局部value reference不可帶出；projection在局部tuple結束前讀出Copy數字。結果200／126／100、140與visits2吻合。

concurrency-stages無lib/dependency/build.rs，早期examples只載自己的明示helper；panic helper test不被其他target載入，未提前載13後文。沒有新增Cargo/Rust執行，沒有量測排程或速度。3張SVG只讀XML/source，不稱browser或讀者驗證。
